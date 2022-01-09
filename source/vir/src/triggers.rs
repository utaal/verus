use crate::ast::{BinaryOp, Ident, Typ, TypX, UnaryOp, UnaryOpr, VarAt, VirErr};
use crate::ast_util::{err_str, err_string};
use crate::context::Ctx;
use crate::sst::{BndX, Exp, ExpX, Trig, Trigs};
use air::ast::Span;
use air::scope_map::ScopeMap;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

// Manual triggers
struct State {
    trigger_vars: HashSet<Ident>, // variables the triggers must cover
    triggers: HashMap<Option<u64>, Vec<Exp>>, // user-specified triggers
    coverage: HashMap<Option<u64>, HashSet<Ident>>, // trigger_vars covered by each trigger
}

fn remove_boxing(exp: &Exp) -> Exp {
    match &exp.x {
        ExpX::UnaryOpr(UnaryOpr::Box(_), e) | ExpX::UnaryOpr(UnaryOpr::Unbox(_), e) => {
            remove_boxing(e)
        }
        _ => exp.clone(),
    }
}

fn check_trigger_expr(exp: &Exp, free_vars: &mut HashSet<Ident>) -> Result<(), VirErr> {
    match &exp.x {
        ExpX::Call(..)
        | ExpX::UnaryOpr(UnaryOpr::Field { .. }, _)
        | ExpX::Unary(UnaryOp::Trigger(_), _) => {}
        // REVIEW: Z3 allows some arithmetic, but it's not clear we want to allow it
        _ => {
            return err_str(&exp.span, "trigger must be a function call or a field access");
        }
    }
    let mut f = |exp: &Exp, _: &mut _| match &exp.x {
        ExpX::Const(_) | ExpX::CallLambda(..) | ExpX::Ctor(..) => Ok(exp.clone()),
        ExpX::Call(_, typs, _) => {
            for typ in typs.iter() {
                let ft = |free_vars: &mut HashSet<Ident>, t: &Typ| match &**t {
                    TypX::TypParam(x) => {
                        free_vars.insert(crate::def::suffix_typ_param_id(x));
                        Ok(t.clone())
                    }
                    _ => Ok(t.clone()),
                };
                crate::ast_visitor::map_typ_visitor_env(typ, free_vars, &ft).unwrap();
            }
            Ok(exp.clone())
        }
        ExpX::Var((x, None)) => {
            free_vars.insert(x.clone());
            Ok(exp.clone())
        }
        ExpX::Var((_, Some(_))) => Ok(exp.clone()),
        ExpX::VarAt(_x, VarAt::Pre) => err_str(&exp.span, "triggers cannot contain old"),
        ExpX::Old(_, _) => panic!("internal error: Old"),
        ExpX::Unary(op, _) => match op {
            UnaryOp::Trigger(_) | UnaryOp::Clip(_) => Ok(exp.clone()),
            UnaryOp::Not => err_str(&exp.span, "triggers cannot contain boolean operators"),
        },
        ExpX::UnaryOpr(op, _) => match op {
            UnaryOpr::Box(_)
            | UnaryOpr::Unbox(_)
            | UnaryOpr::IsVariant { .. }
            | UnaryOpr::TupleField { .. }
            | UnaryOpr::Field { .. } => Ok(exp.clone()),
            UnaryOpr::HasType(_) => panic!("internal error: trigger on HasType"),
        },
        ExpX::Binary(op, _, _) => {
            use BinaryOp::*;
            match op {
                And | Or | Implies | Eq(_) | Ne => {
                    err_str(&exp.span, "triggers cannot contain boolean operators")
                }
                Le | Ge | Lt | Gt => Ok(exp.clone()),
                Add | Sub | Mul | EuclideanDiv | EuclideanMod => Ok(exp.clone()),
            }
        }
        ExpX::If(_, _, _) => err_str(&exp.span, "triggers cannot contain if/else"),
        ExpX::Bind(_, _) => {
            err_str(&exp.span, "triggers cannot contain let/forall/exists/lambda/choose")
        }
    };
    let mut map: ScopeMap<Ident, bool> = ScopeMap::new();
    let _ = crate::sst_visitor::map_exp_visitor_bind(exp, &mut map, &mut f)?;
    Ok(())
}

fn get_manual_triggers(state: &mut State, exp: &Exp) -> Result<(), VirErr> {
    let mut map: ScopeMap<Ident, bool> = ScopeMap::new();
    map.push_scope(false);
    for x in &state.trigger_vars {
        map.insert(x.clone(), true).expect("duplicate bound variables");
    }
    let mut f = |exp: &Exp, map: &mut ScopeMap<Ident, bool>| match &exp.x {
        ExpX::Unary(UnaryOp::Trigger(group), e1) => {
            let mut free_vars: HashSet<Ident> = HashSet::new();
            let e1 = remove_boxing(&e1);
            check_trigger_expr(&e1, &mut free_vars)?;
            for x in &free_vars {
                if map.get(x).cloned() == Some(true) && !state.trigger_vars.contains(x) {
                    // If the trigger contains variables declared by a nested quantifier,
                    // it must be the nested quantifier's trigger, not ours.
                    return Ok(exp.clone());
                }
            }
            if !state.triggers.contains_key(group) {
                state.triggers.insert(*group, Vec::new());
                state.coverage.insert(*group, HashSet::new());
            }
            state.triggers.get_mut(group).unwrap().push(e1.clone());
            for x in &free_vars {
                if state.trigger_vars.contains(x) {
                    state.coverage.get_mut(group).unwrap().insert(x.clone());
                }
            }
            Ok(exp.clone())
        }
        ExpX::Bind(bnd, _) => {
            let bvars: Vec<Ident> = match &bnd.x {
                BndX::Let(binders) => binders.iter().map(|b| b.name.clone()).collect(),
                BndX::Quant(_, binders, _) | BndX::Lambda(binders) => {
                    binders.iter().map(|b| b.name.clone()).collect()
                }
                BndX::Choose(binder, _) => {
                    vec![binder.name.clone()]
                }
            };
            for x in bvars {
                if map.contains_key(&x) {
                    return err_str(&bnd.span, "variable shadowing not yet supported");
                }
            }
            Ok(exp.clone())
        }
        _ => Ok(exp.clone()),
    };
    let _ = crate::sst_visitor::map_exp_visitor_bind(exp, &mut map, &mut f)?;
    map.pop_scope();
    assert_eq!(map.num_scopes(), 0);
    Ok(())
}

pub(crate) fn build_triggers(
    ctx: &Ctx,
    span: &Span,
    vars: &Vec<Ident>,
    exp: &Exp,
) -> Result<Trigs, VirErr> {
    let mut state = State {
        trigger_vars: vars.iter().cloned().collect(),
        triggers: HashMap::new(),
        coverage: HashMap::new(),
    };
    get_manual_triggers(&mut state, exp)?;
    if state.triggers.len() > 0 {
        let mut trigs: Vec<Trig> = Vec::new();
        for (group, trig) in state.triggers {
            for x in vars {
                if !state.coverage[&group].contains(x) {
                    let group_name = match group {
                        None => "".to_string(),
                        Some(id) => format!(" group {}", id),
                    };
                    return err_string(
                        span,
                        format!("trigger{} does not cover variable {}", group_name, x),
                    );
                }
            }
            trigs.push(Arc::new(trig.clone()));
        }
        Ok(Arc::new(trigs))
    } else {
        crate::triggers_auto::build_triggers(ctx, span, vars, exp)
    }
}
