use crate::ast::{
    BinaryOp, Fun, Ident, Idents, IntRange, Mode, Params, Path, Typ, TypX, Typs, UnaryOp, UnaryOpr,
};
use crate::ast_util::{get_field, get_variant};
use crate::context::Ctx;
use crate::def::{
    fun_to_string, path_to_string, prefix_box, prefix_ensures, prefix_fuel_id, prefix_requires,
    suffix_global_id, suffix_local_expr_id, suffix_local_stmt_id, suffix_local_unique_id,
    suffix_typ_param_id, variant_field_ident, variant_ident, SnapPos, SpanKind, Spanned, FUEL_BOOL,
    FUEL_BOOL_DEFAULT, FUEL_DEFAULTS, FUEL_ID, FUEL_PARAM, FUEL_TYPE, POLY, SNAPSHOT_CALL, SUCC,
    SUFFIX_SNAP_JOIN, SUFFIX_SNAP_MUT, SUFFIX_SNAP_WHILE_BEGIN, SUFFIX_SNAP_WHILE_END,
};
use crate::sst::{BndX, Dest, Exp, ExpX, LocalDecl, Stm, StmX, UniqueIdent};
use crate::sst_vars::AssignMap;
use crate::util::vec_map;
use air::ast::{
    BindX, BinderX, Binders, Command, CommandX, Commands, Constant, Decl, DeclX, Expr, ExprX,
    MultiOp, Quant, QueryX, Span, Stmt, StmtX, Trigger, Triggers,
};
use air::ast_util::{
    bool_typ, ident_apply, ident_binder, ident_typ, ident_var, int_typ, mk_and, mk_bind_expr,
    mk_eq, mk_exists, mk_implies, mk_ite, mk_not, mk_or, str_apply, str_ident, str_typ, str_var,
    string_var,
};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

#[inline(always)]
pub(crate) fn fun_to_air_ident(fun: &Fun) -> Ident {
    Arc::new(fun_to_string(fun))
}

#[inline(always)]
pub(crate) fn path_to_air_ident(path: &Path) -> Ident {
    Arc::new(path_to_string(path))
}

pub(crate) fn apply_range_fun(name: &str, range: &IntRange, exprs: Vec<Expr>) -> Expr {
    let mut args = exprs;
    match range {
        IntRange::Int | IntRange::Nat => {}
        IntRange::U(range) | IntRange::I(range) => {
            let bits = Constant::Nat(Arc::new(range.to_string()));
            args.insert(0, Arc::new(ExprX::Const(bits)));
        }
        IntRange::USize | IntRange::ISize => {
            args.insert(0, str_var(crate::def::ARCH_SIZE));
        }
    }
    str_apply(name, &args)
}

pub(crate) fn typ_to_air(_ctx: &Ctx, typ: &Typ) -> air::ast::Typ {
    match &**typ {
        TypX::Int(_) => int_typ(),
        TypX::Bool => bool_typ(),
        TypX::Tuple(_) => panic!("internal error: Tuple should have been removed by ast_simplify"),
        TypX::Datatype(path, _) => ident_typ(&path_to_air_ident(path)),
        TypX::Boxed(_) => str_typ(POLY),
        TypX::TypParam(_) => str_typ(POLY),
        TypX::TypeId => str_typ(crate::def::TYPE),
    }
}

// SMT-level type identifiers.
// We currently rely on these type identifiers for:
// 1) Axioms about unboxing integer refinement types (nat, u8, etc.)
// 2) The box(unbox(x)) == x axiom
pub fn typ_to_id(typ: &Typ) -> Expr {
    match &**typ {
        TypX::Int(range) => match range {
            IntRange::Int => str_var(crate::def::TYPE_ID_INT),
            IntRange::Nat => str_var(crate::def::TYPE_ID_NAT),
            IntRange::U(_) | IntRange::USize => {
                apply_range_fun(crate::def::TYPE_ID_UINT, range, vec![])
            }
            IntRange::I(_) | IntRange::ISize => {
                apply_range_fun(crate::def::TYPE_ID_SINT, range, vec![])
            }
        },
        TypX::Bool => str_var(crate::def::TYPE_ID_BOOL),
        TypX::Tuple(_) => panic!("internal error: Tuple should have been removed by ast_simplify"),
        TypX::Datatype(path, typs) => datatype_id(path, typs),
        TypX::Boxed(_) => panic!("internal error: type arguments should be unboxed"),
        TypX::TypParam(x) => ident_var(&suffix_typ_param_id(x)),
        TypX::TypeId => panic!("internal error: typ_to_id of TypeId"),
    }
}

pub(crate) fn datatype_id(path: &Path, typs: &Typs) -> Expr {
    let f_name = crate::def::prefix_type_id(path);
    air::ast_util::ident_apply_or_var(&f_name, &Arc::new(vec_map(&**typs, typ_to_id)))
}

pub(crate) fn datatype_has_type(path: &Path, typs: &Typs, expr: &Expr) -> Expr {
    str_apply(crate::def::HAS_TYPE, &vec![expr.clone(), datatype_id(path, typs)])
}

// If expr has type typ, what can we assume to be true about expr?
// For refinement types, transparent datatypes potentially containing refinement types,
// and type variables potentially instantiated with refinement types, return Some invariant.
// For non-refinement types and abstract types, return None,
// since the SMT sorts for these types can express the types precisely with no need for refinement.
pub(crate) fn typ_invariant(ctx: &Ctx, typ: &Typ, expr: &Expr) -> Option<Expr> {
    // Should be kept in sync with vir::context::datatypes_invs
    match &**typ {
        TypX::Int(IntRange::Int) => None,
        TypX::Int(IntRange::Nat) => {
            let zero = Arc::new(ExprX::Const(Constant::Nat(Arc::new("0".to_string()))));
            Some(Arc::new(ExprX::Binary(air::ast::BinaryOp::Le, zero, expr.clone())))
        }
        TypX::Int(range) => {
            let f_name = match range {
                IntRange::Int => panic!("internal error: Int"),
                IntRange::Nat => panic!("internal error: Int"),
                IntRange::U(_) | IntRange::USize => crate::def::U_INV,
                IntRange::I(_) | IntRange::ISize => crate::def::I_INV,
            };
            Some(apply_range_fun(&f_name, &range, vec![expr.clone()]))
        }
        TypX::Datatype(path, typs) if ctx.datatypes_with_invariant.contains(path) => {
            let box_expr = ident_apply(&prefix_box(&path), &vec![expr.clone()]);
            Some(datatype_has_type(path, typs, &box_expr))
        }
        TypX::TypParam(x) => Some(str_apply(
            crate::def::HAS_TYPE,
            &vec![expr.clone(), ident_var(&suffix_typ_param_id(&x))],
        )),
        _ => None,
    }
}

pub(crate) fn ctor_to_apply<'a>(
    ctx: &'a Ctx,
    path: &Path,
    variant: &Ident,
    binders: &'a Binders<Exp>,
) -> (Ident, impl Iterator<Item = &'a Arc<BinderX<Exp>>>) {
    let fields = &get_variant(&ctx.global.datatypes[path], variant).a;
    (variant_ident(path, &variant), fields.iter().map(move |f| get_field(binders, &f.name)))
}

pub(crate) fn constant_to_expr(_ctx: &Ctx, constant: &crate::ast::Constant) -> Expr {
    match constant {
        crate::ast::Constant::Bool(b) => Arc::new(ExprX::Const(Constant::Bool(*b))),
        crate::ast::Constant::Nat(s) => Arc::new(ExprX::Const(Constant::Nat(s.clone()))),
    }
}

pub(crate) fn exp_to_expr(ctx: &Ctx, exp: &Exp) -> Expr {
    match &exp.x {
        ExpX::Const(c) => {
            let expr = constant_to_expr(ctx, c);
            expr
        }
        ExpX::Var(x) => string_var(&suffix_local_unique_id(x)),
        ExpX::Old(span, x) => Arc::new(ExprX::Old(span.clone(), suffix_local_stmt_id(x))),
        ExpX::Call(x, typs, args) => {
            let name = suffix_global_id(&fun_to_air_ident(&x));
            let mut exprs: Vec<Expr> = vec_map(typs, typ_to_id);
            for arg in args.iter() {
                exprs.push(exp_to_expr(ctx, arg));
            }
            ident_apply(&name, &exprs)
        }
        ExpX::Ctor(path, variant, binders) => {
            let (variant, args) = ctor_to_apply(ctx, path, variant, binders);
            let args = args.map(|b| exp_to_expr(ctx, &b.a)).collect::<Vec<_>>();
            Arc::new(ExprX::Apply(variant, Arc::new(args)))
        }
        ExpX::Unary(op, exp) => match op {
            UnaryOp::Not => mk_not(&exp_to_expr(ctx, exp)),
            UnaryOp::Trigger(_) => exp_to_expr(ctx, exp),
            UnaryOp::Clip(IntRange::Int) => exp_to_expr(ctx, exp),
            UnaryOp::Clip(range) => {
                let expr = exp_to_expr(ctx, exp);
                let f_name = match range {
                    IntRange::Int => panic!("internal error: Int"),
                    IntRange::Nat => crate::def::NAT_CLIP,
                    IntRange::U(_) | IntRange::USize => crate::def::U_CLIP,
                    IntRange::I(_) | IntRange::ISize => crate::def::I_CLIP,
                };
                apply_range_fun(&f_name, &range, vec![expr])
            }
        },
        ExpX::UnaryOpr(op, exp) => match op {
            UnaryOpr::Box(typ) => {
                let expr = exp_to_expr(ctx, exp);
                let f_name = match &**typ {
                    TypX::Bool => str_ident(crate::def::BOX_BOOL),
                    TypX::Int(_) => str_ident(crate::def::BOX_INT),
                    TypX::Datatype(path, _) => prefix_box(&path),
                    TypX::Tuple(_) => panic!("internal error: Box(Tuple)"),
                    TypX::Boxed(_) => panic!("internal error: Box(Boxed)"),
                    TypX::TypParam(_) => panic!("internal error: Box(TypParam)"),
                    TypX::TypeId => panic!("internal error: Box(TypeId)"),
                };
                ident_apply(&f_name, &vec![expr])
            }
            UnaryOpr::Unbox(typ) => {
                let expr = exp_to_expr(ctx, exp);
                let f_name = match &**typ {
                    TypX::Bool => str_ident(crate::def::UNBOX_BOOL),
                    TypX::Int(_) => str_ident(crate::def::UNBOX_INT),
                    TypX::Datatype(path, _) => crate::def::prefix_unbox(&path),
                    TypX::Tuple(_) => panic!("internal error: Box(Tuple)"),
                    TypX::Boxed(_) => panic!("internal error: Unbox(Boxed)"),
                    TypX::TypParam(_) => panic!("internal error: Unbox(TypParam)"),
                    TypX::TypeId => panic!("internal error: Unbox(TypeId)"),
                };
                ident_apply(&f_name, &vec![expr])
            }
            UnaryOpr::IsVariant { datatype, variant } => {
                let expr = exp_to_expr(ctx, exp);
                let name = Arc::new(format!("is-{}", variant_ident(datatype, variant)));
                Arc::new(ExprX::Apply(name, Arc::new(vec![expr])))
            }
            UnaryOpr::TupleField { .. } => {
                panic!("internal error: TupleField should have been removed before here")
            }
            UnaryOpr::Field { datatype, variant, field } => {
                // TODO: this should include datatype, variant in the function name
                let expr = exp_to_expr(ctx, exp);
                Arc::new(ExprX::Apply(
                    variant_field_ident(datatype, variant, field),
                    Arc::new(vec![expr]),
                ))
            }
        },
        ExpX::Binary(op, lhs, rhs) => {
            let lh = exp_to_expr(ctx, lhs);
            let rh = exp_to_expr(ctx, rhs);
            let expx = match op {
                BinaryOp::And => {
                    return mk_and(&vec![lh, rh]);
                }
                BinaryOp::Or => {
                    return mk_or(&vec![lh, rh]);
                }
                BinaryOp::Implies => {
                    return mk_implies(&lh, &rh);
                }
                BinaryOp::Add => ExprX::Multi(MultiOp::Add, Arc::new(vec![lh, rh])),
                BinaryOp::Sub => ExprX::Multi(MultiOp::Sub, Arc::new(vec![lh, rh])),
                BinaryOp::Mul => ExprX::Multi(MultiOp::Mul, Arc::new(vec![lh, rh])),
                BinaryOp::Ne => {
                    let eq = ExprX::Binary(air::ast::BinaryOp::Eq, lh, rh);
                    ExprX::Unary(air::ast::UnaryOp::Not, Arc::new(eq))
                }
                _ => {
                    let aop = match op {
                        BinaryOp::And => panic!("internal error"),
                        BinaryOp::Or => panic!("internal error"),
                        BinaryOp::Implies => panic!("internal error"),
                        BinaryOp::Eq(_) => air::ast::BinaryOp::Eq,
                        BinaryOp::Ne => panic!("internal error"),
                        BinaryOp::Le => air::ast::BinaryOp::Le,
                        BinaryOp::Ge => air::ast::BinaryOp::Ge,
                        BinaryOp::Lt => air::ast::BinaryOp::Lt,
                        BinaryOp::Gt => air::ast::BinaryOp::Gt,
                        BinaryOp::Add => panic!("internal error"),
                        BinaryOp::Sub => panic!("internal error"),
                        BinaryOp::Mul => panic!("internal error"),
                        BinaryOp::EuclideanDiv => air::ast::BinaryOp::EuclideanDiv,
                        BinaryOp::EuclideanMod => air::ast::BinaryOp::EuclideanMod,
                    };
                    ExprX::Binary(aop, lh, rh)
                }
            };
            Arc::new(expx)
        }
        ExpX::If(e1, e2, e3) => {
            mk_ite(&exp_to_expr(ctx, e1), &exp_to_expr(ctx, e2), &exp_to_expr(ctx, e3))
        }
        ExpX::Bind(bnd, exp) => match &bnd.x {
            BndX::Let(binders) => {
                let expr = exp_to_expr(ctx, exp);
                let binders = vec_map(&*binders, |b| {
                    Arc::new(BinderX {
                        name: suffix_local_expr_id(&b.name),
                        a: exp_to_expr(ctx, &b.a),
                    })
                });
                air::ast_util::mk_let(&binders, &expr)
            }
            BndX::Quant(quant, binders, trigs) => {
                let expr = exp_to_expr(ctx, exp);
                let mut invs: Vec<Expr> = Vec::new();
                for binder in binders.iter() {
                    let typ_inv = typ_invariant(
                        ctx,
                        &binder.a,
                        &ident_var(&suffix_local_expr_id(&binder.name)),
                    );
                    if let Some(inv) = typ_inv {
                        invs.push(inv);
                    }
                }
                let inv = mk_and(&invs);
                let expr = match quant {
                    Quant::Forall => mk_implies(&inv, &expr),
                    Quant::Exists => mk_and(&vec![inv, expr]),
                };
                let binders = vec_map(&*binders, |b| {
                    let name = match &*b.a {
                        // allow quantifiers over type parameters, generated for export_as_global_forall
                        TypX::TypeId => suffix_typ_param_id(&b.name),
                        _ => suffix_local_expr_id(&b.name),
                    };
                    Arc::new(BinderX { name, a: typ_to_air(ctx, &b.a) })
                });
                let triggers =
                    vec_map(&*trigs, |trig| Arc::new(vec_map(trig, |x| exp_to_expr(ctx, x))));
                air::ast_util::mk_quantifier(*quant, &binders, &triggers, &expr)
            }
        },
    }
}

struct State {
    local_shared: Vec<Decl>, // shared between all queries for a single function
    commands: Vec<Command>,
    snapshot_count: u32, // Used to ensure unique Idents for each snapshot
    sids: Vec<Ident>, // a stack of snapshot ids, the top one should dominate the current position in the AST
    snap_map: Vec<(Span, SnapPos)>, // Maps each statement's span to the closest dominating snapshot's ID
    assign_map: AssignMap, // Maps Maps each statement's span to the assigned variables (that can potentially be queried)
}

impl State {
    /// get the current sid (top of the scope stack)
    fn get_current_sid(&self) -> Ident {
        let last = self.sids.last().unwrap();
        return last.clone();
    }

    /// copy the current sid into a new scope (when entering a block)
    fn push_scope(&mut self) {
        let sid = self.get_current_sid();
        self.sids.push(sid);
    }

    /// pop off the scope (when exiting a block)
    fn pop_scope(&mut self) {
        self.sids.pop();
    }

    fn get_new_sid(&mut self, suffix: &str) -> Ident {
        self.snapshot_count += 1;
        return Arc::new(format!("{}{}", self.snapshot_count, suffix));
    }

    /// replace the current sid (without changing scope depth)
    fn update_current_sid(&mut self, suffix: &str) -> Ident {
        let sid = self.get_new_sid(suffix);
        self.sids.pop();
        self.sids.push(sid.clone());
        return sid;
    }

    fn get_assigned_set(&self, stm: &Stm) -> HashSet<Arc<String>> {
        if let Some(s) = self.assign_map.get(&Arc::as_ptr(stm)) {
            return s.clone();
        }
        return HashSet::new();
    }

    fn map_span(&mut self, stm: &Stm, kind: SpanKind) {
        let spos = SnapPos { snapshot_id: self.get_current_sid(), kind: kind };
        let aset = self.get_assigned_set(stm);
        println!("{:?} {:?}", stm.span, aset);
        self.snap_map.push((stm.span.clone(), spos));
    }

    // fn map_full_span(&mut self, stm: &Stm) {
    //     let spos = SnapPos::Full(self.get_current_sid());
    //     let aset = self.get_assigned_set(stm);
    //     println!("{:?} {:?}", stm.span, aset);
    //     self.snap_map.push((stm.span.clone(), spos));
    // }

    // fn map_end_span(&mut self, stm: &Stm) {
    //     let spos = SnapPos::End(self.get_current_sid());
    //     let aset = self.get_assigned_set(stm);
    //     println!("{:?} {:?}", stm.span, aset);
    //     self.snap_map.push((stm.span.clone(), spos));
    // }
}

fn assume_var(span: &Span, x: &UniqueIdent, exp: &Exp) -> Stm {
    let x_var = Spanned::new(span.clone(), ExpX::Var(x.clone()));
    let eq = Spanned::new(span.clone(), ExpX::Binary(BinaryOp::Eq(Mode::Spec), x_var, exp.clone()));
    Spanned::new(span.clone(), StmX::Assume(eq))
}

fn one_stmt(stmts: Vec<Stmt>) -> Stmt {
    if stmts.len() == 1 { stmts[0].clone() } else { Arc::new(StmtX::Block(Arc::new(stmts))) }
}

fn stm_to_stmts(ctx: &Ctx, state: &mut State, stm: &Stm) -> Vec<Stmt> {
    match &stm.x {
        StmX::Call(x, typs, args, dest) => {
            let mut stmts: Vec<Stmt> = Vec::new();
            let func = &ctx.func_map[x];
            if func.x.require.len() > 0 {
                let f_req = prefix_requires(&fun_to_air_ident(&func.x.name));
                let mut req_args = vec_map(typs, typ_to_id);
                for arg in args.iter() {
                    req_args.push(exp_to_expr(ctx, arg));
                }
                let e_req = Arc::new(ExprX::Apply(f_req, Arc::new(req_args)));
                let description = match &func.x.attrs.custom_req_err {
                    None => Some("precondition not satisfied".to_string()),
                    Some(s) => Some(s.clone()),
                };
                let option_span = Arc::new(Some(Span { description, ..stm.span.clone() }));
                stmts.push(Arc::new(StmtX::Assert(option_span, e_req)));
            }
            let mut ens_args: Vec<Expr> = vec_map(typs, typ_to_id);
            match dest {
                None => {
                    for arg in args.iter() {
                        ens_args.push(exp_to_expr(ctx, arg));
                    }
                    if ctx.debug {
                        state.map_span(&stm, SpanKind::Full);
                    }
                }
                Some(Dest { var, is_init }) => {
                    let x = suffix_local_unique_id(&var);
                    let mut overwrite = false;
                    for arg in args.iter() {
                        let arg_x = crate::sst_visitor::map_exp_visitor(arg, &mut |e| match &e.x {
                            ExpX::Var(x) if x == var => {
                                overwrite = true;
                                Spanned::new(
                                    arg.span.clone(),
                                    ExpX::Old(str_ident(SNAPSHOT_CALL), x.0.clone()),
                                )
                            }
                            _ => e.clone(),
                        });
                        ens_args.push(exp_to_expr(ctx, &arg_x));
                    }
                    if overwrite {
                        stmts.push(Arc::new(StmtX::Snapshot(str_ident(SNAPSHOT_CALL))));
                    }
                    ens_args.push(Arc::new(ExprX::Var(x.clone())));
                    if !*is_init {
                        let havoc = StmtX::Havoc(x.clone());
                        stmts.push(Arc::new(havoc));
                    }
                    if ctx.debug {
                        // Add a snapshot after we modify the destination
                        let sid = state.update_current_sid(SUFFIX_SNAP_MUT);
                        // Update the snap_map so that it reflects the state _after_ the
                        // statement takes effect.
                        state.map_span(&stm, SpanKind::Full);
                        let snapshot = Arc::new(StmtX::Snapshot(sid.clone()));
                        stmts.push(snapshot);
                    }
                }
            }
            if ctx.funcs_with_ensure_predicate.contains(&func.x.name) {
                let f_ens = prefix_ensures(&fun_to_air_ident(&func.x.name));
                let e_ens = Arc::new(ExprX::Apply(f_ens, Arc::new(ens_args)));
                stmts.push(Arc::new(StmtX::Assume(e_ens)));
            }
            vec![Arc::new(StmtX::Block(Arc::new(stmts)))] // wrap in block for readability
        }
        StmX::Assert(expr) => {
            let air_expr = exp_to_expr(ctx, &expr);
            let option_span = Arc::new(Some(stm.span.clone()));
            if ctx.debug {
                state.map_span(&stm, SpanKind::Full);
            }
            vec![Arc::new(StmtX::Assert(option_span, air_expr))]
        }
        StmX::Assume(expr) => {
            if ctx.debug {
                state.map_span(&stm, SpanKind::Full);
            }
            vec![Arc::new(StmtX::Assume(exp_to_expr(ctx, &expr)))]
        }
        StmX::Assign { lhs, rhs, is_init: true } => {
            stm_to_stmts(ctx, state, &assume_var(&stm.span, lhs, rhs))
        }
        StmX::Assign { lhs, rhs, is_init: false } => {
            let mut stmts: Vec<Stmt> = Vec::new();
            let name = suffix_local_unique_id(lhs);
            stmts.push(Arc::new(StmtX::Assign(name, exp_to_expr(ctx, rhs))));
            if ctx.debug {
                // Add a snapshot after we modify the destination
                let sid = state.update_current_sid(SUFFIX_SNAP_MUT);
                let snapshot = Arc::new(StmtX::Snapshot(sid.clone()));
                stmts.push(snapshot);
                // Update the snap_map so that it reflects the state _after_ the
                // statement takes effect.
                state.map_span(&stm, SpanKind::Full);
            }
            stmts
        }
        StmX::DeadEnd(s) => {
            vec![Arc::new(StmtX::DeadEnd(one_stmt(stm_to_stmts(ctx, state, s))))]
        }
        StmX::If(cond, lhs, rhs) => {
            let pos_cond = exp_to_expr(ctx, &cond);
            let neg_cond = Arc::new(ExprX::Unary(air::ast::UnaryOp::Not, pos_cond.clone()));
            let pos_assume = Arc::new(StmtX::Assume(pos_cond));
            let neg_assume = Arc::new(StmtX::Assume(neg_cond));
            let mut lhss = stm_to_stmts(ctx, state, lhs);
            let mut rhss = match rhs {
                None => vec![],
                Some(rhs) => stm_to_stmts(ctx, state, rhs),
            };
            lhss.insert(0, pos_assume);
            rhss.insert(0, neg_assume);
            let lblock = Arc::new(StmtX::Block(Arc::new(lhss)));
            let rblock = Arc::new(StmtX::Block(Arc::new(rhss)));
            let mut stmts = vec![Arc::new(StmtX::Switch(Arc::new(vec![lblock, rblock])))];
            if ctx.debug {
                // Add a snapshot for the state after we join the lhs and rhs back together
                let sid = state.update_current_sid(SUFFIX_SNAP_JOIN);
                let snapshot = Arc::new(StmtX::Snapshot(sid.clone()));
                stmts.push(snapshot);
                state.map_span(&stm, SpanKind::End);
            }
            stmts
        }
        StmX::While { cond, body, invs, typ_inv_vars, modified_vars } => {
            let pos_cond = exp_to_expr(ctx, &cond);
            let neg_cond = Arc::new(ExprX::Unary(air::ast::UnaryOp::Not, pos_cond.clone()));
            let pos_assume = Arc::new(DeclX::Axiom(pos_cond));
            let neg_assume = Arc::new(StmtX::Assume(neg_cond));
            let invs: Vec<(Span, Expr)> =
                invs.iter().map(|e| (e.span.clone(), exp_to_expr(ctx, e))).collect();

            let entry_snap_id = if ctx.debug {
                // Add a snapshot to capture the start of the while loop
                // We add the snapshot via Block to avoid copying the entire AST of the loop body
                let entry_snap = state.update_current_sid(SUFFIX_SNAP_WHILE_BEGIN);
                Some(entry_snap)
            } else {
                None
            };

            let mut air_body = stm_to_stmts(ctx, state, body);

            /*
            Generate a separate SMT query for the loop body.
            Rationale: large functions with while loops tend to be slow to verify.
            Therefore, it's good to try to factor large functions
            into smaller, easier-to-verify pieces.
            Since we have programmer-supplied invariants anyway,
            this is a good place for such refactoring.
            This isn't necessarily a benefit for small functions or small loops,
            but in practice, verification for large functions and large loops are slow
            enough that programmers often do this refactoring by hand anyway,
            so it's a benefit when verification gets hard, which is arguably what matters most.
            (The downside: the programmer might have to write more complete invariants,
            but this is part of the point: the invariants specify a precise interface
            between the outer function and the inner loop body, so we don't have to import
            the outer function's entire context into the verification of the loop body,
            which would slow verification of the loop body.)
            */
            let mut local = state.local_shared.clone();
            for (x, typ) in typ_inv_vars.iter() {
                let typ_inv = typ_invariant(ctx, typ, &ident_var(&suffix_local_unique_id(x)));
                if let Some(expr) = typ_inv {
                    local.push(Arc::new(DeclX::Axiom(expr)));
                }
            }
            for (_, inv) in invs.iter() {
                local.push(Arc::new(DeclX::Axiom(inv.clone())));
            }
            local.push(pos_assume);
            for (span, inv) in invs.iter() {
                let description = Some("invariant not satisfied at end of loop body".to_string());
                let option_span = Arc::new(Some(Span { description, ..span.clone() }));
                let inv_stmt = StmtX::Assert(option_span, inv.clone());
                air_body.push(Arc::new(inv_stmt));
            }
            let assertion = one_stmt(air_body);

            let assertion = if !ctx.debug {
                assertion
            } else {
                // Update the snap_map to associate the start of the while loop with the new snapshot
                let entry_snap_id = entry_snap_id.unwrap(); // Always Some if ctx.debug
                let snapshot: Stmt = Arc::new(StmtX::Snapshot(entry_snap_id.clone()));
                state.map_span(&body, SpanKind::Start);
                let block_contents: Vec<Stmt> = vec![snapshot, assertion];
                Arc::new(StmtX::Block(Arc::new(block_contents)))
            };

            let query = Arc::new(QueryX { local: Arc::new(local), assertion });
            state.commands.push(Arc::new(CommandX::CheckValid(query)));

            // At original site of while loop, assert invariant, havoc, assume invariant + neg_cond
            let mut stmts: Vec<Stmt> = Vec::new();
            for (span, inv) in invs.iter() {
                let description = Some("invariant not satisfied before loop".to_string());
                let option_span = Arc::new(Some(Span { description, ..span.clone() }));
                let inv_stmt = StmtX::Assert(option_span, inv.clone());
                stmts.push(Arc::new(inv_stmt));
            }
            for x in modified_vars.iter() {
                stmts.push(Arc::new(StmtX::Havoc(suffix_local_unique_id(&x))));
            }
            for (x, typ) in typ_inv_vars.iter() {
                if modified_vars.contains(x) {
                    let typ_inv = typ_invariant(ctx, typ, &ident_var(&suffix_local_unique_id(x)));
                    if let Some(expr) = typ_inv {
                        stmts.push(Arc::new(StmtX::Assume(expr)));
                    }
                }
            }
            for (_, inv) in invs.iter() {
                let inv_stmt = StmtX::Assume(inv.clone());
                stmts.push(Arc::new(inv_stmt));
            }
            stmts.push(neg_assume);
            if ctx.debug {
                // Add a snapshot for the state after we emerge from the while loop
                let sid = state.update_current_sid(SUFFIX_SNAP_WHILE_END);
                // Update the snap_map so that it reflects the state _after_ the
                // statement takes effect.
                state.map_span(&stm, SpanKind::End);
                let snapshot = Arc::new(StmtX::Snapshot(sid));
                stmts.push(snapshot);
            }
            stmts
        }
        StmX::Fuel(x, fuel) => {
            let mut stmts: Vec<Stmt> = Vec::new();
            if *fuel >= 1 {
                // (assume (fuel_bool fuel%f))
                let id_fuel = prefix_fuel_id(&fun_to_air_ident(&x));
                let expr_fuel_bool = str_apply(&FUEL_BOOL, &vec![ident_var(&id_fuel)]);
                stmts.push(Arc::new(StmtX::Assume(expr_fuel_bool)));
            }
            if *fuel >= 2 {
                // (assume (exists ((fuel Fuel)) (= fuel_nat%f (succ ... succ fuel))))
                let mut added_fuel = str_var(FUEL_PARAM);
                for _ in 0..*fuel - 1 {
                    added_fuel = str_apply(SUCC, &vec![added_fuel]);
                }
                let eq = mk_eq(
                    &ident_var(&crate::def::prefix_fuel_nat(&fun_to_air_ident(&x))),
                    &added_fuel,
                );
                let binder = ident_binder(&str_ident(FUEL_PARAM), &str_typ(FUEL_TYPE));
                stmts.push(Arc::new(StmtX::Assume(mk_exists(&vec![binder], &vec![], &eq))));
            }
            if ctx.debug {
                state.map_span(&stm, SpanKind::Full);
            }
            stmts
        }
        StmX::Block(stms) => {
            if ctx.debug {
                state.push_scope();
                state.map_span(&stm, SpanKind::Start);
            }
            let stmts = stms.iter().map(|s| stm_to_stmts(ctx, state, s)).flatten().collect();
            if ctx.debug {
                state.pop_scope();
            }
            stmts
        }
    }
}

fn set_fuel(local: &mut Vec<Decl>, hidden: &Vec<Fun>) {
    let fuel_expr = if hidden.len() == 0 {
        str_var(&FUEL_DEFAULTS)
    } else {
        let mut disjuncts: Vec<Expr> = Vec::new();
        let id = str_ident("id");
        let x_id = ident_var(&id);

        // (= (fuel_bool id) (fuel_bool_default id))
        let fuel_bool = str_apply(&FUEL_BOOL, &vec![x_id.clone()]);
        let fuel_bool_default = str_apply(&FUEL_BOOL_DEFAULT, &vec![x_id.clone()]);
        let eq = air::ast::BinaryOp::Eq;
        disjuncts.push(Arc::new(ExprX::Binary(eq, fuel_bool.clone(), fuel_bool_default)));

        // ... || id == hidden1 || id == hidden2 || ...
        for hide in hidden {
            let x_hide = ident_var(&prefix_fuel_id(&fun_to_air_ident(hide)));
            disjuncts.push(Arc::new(ExprX::Binary(air::ast::BinaryOp::Eq, x_id.clone(), x_hide)));
        }

        // (forall ((id FuelId)) ...)
        let trigger: Trigger = Arc::new(vec![fuel_bool.clone()]);
        let triggers: Triggers = Arc::new(vec![trigger]);
        let binders: Binders<air::ast::Typ> = Arc::new(vec![ident_binder(&id, &str_typ(FUEL_ID))]);
        let bind = Arc::new(BindX::Quant(Quant::Forall, binders, triggers));
        let or = Arc::new(ExprX::Multi(air::ast::MultiOp::Or, Arc::new(disjuncts)));
        mk_bind_expr(&bind, &or)
    };
    local.push(Arc::new(DeclX::Axiom(fuel_expr)));
}

pub fn body_stm_to_air(
    ctx: &Ctx,
    typ_params: &Idents,
    params: &Params,
    local_decls: &Vec<LocalDecl>,
    hidden: &Vec<Fun>,
    reqs: &Vec<Exp>,
    enss: &Vec<Exp>,
    stm: &Stm,
) -> (Commands, Vec<(Span, SnapPos)>) {
    // Verifying a single function can generate multiple SMT queries.
    // Some declarations (local_shared) are shared among the queries.
    // Others are private to each query.
    let mut local_shared: Vec<Decl> = Vec::new();
    for x in typ_params.iter() {
        local_shared
            .push(Arc::new(DeclX::Const(suffix_typ_param_id(&x), str_typ(crate::def::TYPE))));
    }
    for decl in local_decls {
        local_shared.push(if decl.mutable {
            Arc::new(DeclX::Var(suffix_local_unique_id(&decl.ident), typ_to_air(ctx, &decl.typ)))
        } else {
            Arc::new(DeclX::Const(suffix_local_unique_id(&decl.ident), typ_to_air(ctx, &decl.typ)))
        });
    }

    set_fuel(&mut local_shared, hidden);

    let mut declared: HashMap<UniqueIdent, Typ> = HashMap::new();
    let mut assigned: HashSet<UniqueIdent> = HashSet::new();
    for param in params.iter() {
        declared.insert((param.x.name.clone(), Some(0)), param.x.typ.clone());
        assigned.insert((param.x.name.clone(), Some(0)));
    }
    for decl in local_decls {
        declared.insert(decl.ident.clone(), decl.typ.clone());
    }

    let initial_sid = Arc::new("0_entry".to_string());

    let mut state = State {
        local_shared,
        commands: Vec::new(),
        snapshot_count: 0,
        sids: vec![initial_sid.clone()],
        snap_map: Vec::new(),
        assign_map: HashMap::new(),
    };

    // println!("assign map {:?}", stm);

    let stm = crate::sst_vars::stm_assign(
        &mut state.assign_map,
        &declared,
        &mut assigned,
        &mut HashSet::new(),
        stm,
    );
    let mut stmts = stm_to_stmts(ctx, &mut state, &stm);

    if ctx.debug {
        let snapshot = Arc::new(StmtX::Snapshot(initial_sid));
        let mut new_stmts = vec![snapshot];
        new_stmts.append(&mut stmts);
        stmts = new_stmts;
    }

    let mut local = state.local_shared.clone();

    for ens in enss {
        let description = Some("postcondition not satisfied".to_string());
        let option_span = Arc::new(Some(Span { description, ..ens.span.clone() }));
        let ens_stmt = StmtX::Assert(option_span, exp_to_expr(ctx, ens));
        stmts.push(Arc::new(ens_stmt));
    }
    let assertion = one_stmt(stmts);

    for param in params.iter() {
        let typ_inv =
            typ_invariant(ctx, &param.x.typ, &ident_var(&suffix_local_stmt_id(&param.x.name)));
        if let Some(expr) = typ_inv {
            local.push(Arc::new(DeclX::Axiom(expr)));
        }
    }

    for req in reqs {
        local.push(Arc::new(DeclX::Axiom(exp_to_expr(ctx, req))));
    }

    let query = Arc::new(QueryX { local: Arc::new(local), assertion });
    state.commands.push(Arc::new(CommandX::CheckValid(query)));
    (Arc::new(state.commands), state.snap_map)
}
