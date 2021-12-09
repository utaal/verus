/// 1) Optimize generated SMT by pruning unreachable declarations and definitions.
///    This is strictly an optimization; it should not affect the SMT validity.
/// 2) Also remove any export_as_global_forall from any modules that are unreachable
///    from this module.  This could, in principle, result in incompleteness.
use crate::ast::{
    CallTarget, Datatype, Expr, ExprX, Fun, Function, Ident, Krate, KrateX, Mode, Path, Stmt, Typ,
    TypX, Visibility,
};
use crate::ast_util::is_visible_to;
use crate::ast_visitor::map_expr_visitor;
use crate::datatype_to_air::is_datatype_transparent;
use crate::def::Spanned;
use air::scope_map::ScopeMap;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

struct Ctxt {
    function_map: HashMap<Fun, Function>,
    datatype_map: HashMap<Path, Datatype>,
    all_functions_in_each_module: HashMap<Path, Vec<Fun>>,
}

#[derive(Default)]
struct State {
    reached_functions: HashSet<Fun>,
    reached_datatypes: HashSet<Path>,
    reached_modules: HashSet<Path>,
    worklist_functions: Vec<Fun>,
    worklist_datatypes: Vec<Path>,
    worklist_modules: Vec<Path>,
}

fn reach<A: std::hash::Hash + std::cmp::Eq + Clone>(
    reached: &mut HashSet<A>,
    worklist: &mut Vec<A>,
    id: &A,
) {
    if !reached.contains(id) {
        reached.insert(id.clone());
        worklist.push(id.clone());
    }
}

fn reach_function(ctxt: &Ctxt, state: &mut State, name: &Fun) {
    if ctxt.function_map.contains_key(name) {
        reach(&mut state.reached_functions, &mut state.worklist_functions, name);
        let module_path = name.path.pop_segment();
        reach(&mut state.reached_modules, &mut state.worklist_modules, &module_path);
    }
}

fn reach_datatype(ctxt: &Ctxt, state: &mut State, path: &Path) {
    if ctxt.datatype_map.contains_key(path) {
        reach(&mut state.reached_datatypes, &mut state.worklist_datatypes, path);
        let module_path = path.pop_segment();
        reach(&mut state.reached_modules, &mut state.worklist_modules, &module_path);
    }
}

fn traverse_reachable(ctxt: &Ctxt, state: &mut State) {
    loop {
        let ft = |state: &mut State, t: &Typ| {
            match &**t {
                TypX::Datatype(path, _) => reach_datatype(ctxt, state, path),
                _ => {}
            }
            Ok(t.clone())
        };
        if let Some(f) = state.worklist_functions.pop() {
            let function = &ctxt.function_map[&f];
            let fe = |state: &mut State, _: &mut ScopeMap<Ident, Typ>, e: &Expr| {
                match &e.x {
                    ExprX::Call(CallTarget::Static(name, _), _) => {
                        reach_function(ctxt, state, name)
                    }
                    ExprX::Ctor(path, _, _, _) => reach_datatype(ctxt, state, path),
                    _ => {}
                }
                Ok(e.clone())
            };
            let fs = |_: &mut State, _: &mut ScopeMap<Ident, Typ>, s: &Stmt| Ok(vec![s.clone()]);
            let mut map: ScopeMap<Ident, Typ> = ScopeMap::new();
            crate::ast_visitor::map_function_visitor_env(&function, &mut map, state, &fe, &fs, &ft)
                .unwrap();
            continue;
        }
        if let Some(d) = state.worklist_datatypes.pop() {
            let datatype = &ctxt.datatype_map[&d];
            crate::ast_visitor::map_datatype_visitor_env(&datatype, state, &ft).unwrap();
            continue;
        }
        if let Some(m) = state.worklist_modules.pop() {
            if let Some(fs) = ctxt.all_functions_in_each_module.get(&m) {
                for f in fs {
                    let function = &ctxt.function_map[f];
                    if function.x.attrs.export_as_global_forall {
                        // If we reach m, we reach all export_as_global_forall functions in m
                        reach_function(ctxt, state, f);
                    }
                }
            }
            continue;
        }
        break;
    }
}

pub fn prune_krate_for_module(krate: &Krate, module: &Path) -> Krate {
    let mut state: State = Default::default();
    state.reached_modules.insert(module.clone());
    state.worklist_modules.push(module.clone());

    // Collect all functions that our module reveals:
    let mut revealed_functions: HashSet<Fun> = HashSet::new();
    for f in &krate.functions {
        match (&f.x.visibility.owning_module, &f.x.body) {
            (Some(path), Some(body)) if path == module => {
                map_expr_visitor(body, &mut |e: &Expr| {
                    match &e.x {
                        ExprX::Fuel(path, fuel) if *fuel > 0 => {
                            revealed_functions.insert(path.clone());
                        }
                        _ => {}
                    }
                    Ok(e.clone())
                })
                .unwrap();
            }
            _ => {}
        }
    }

    // Collect functions and datatypes,
    // pruning all bodies and variants that are not visible to our module
    let mut functions: Vec<Function> = Vec::new();
    let mut datatypes: Vec<Datatype> = Vec::new();
    for f in &krate.functions {
        match &f.x.visibility.owning_module {
            Some(path) if path == module => {
                // our function
                functions.push(f.clone());
                state.reached_functions.insert(f.x.name.clone());
                state.worklist_functions.push(f.x.name.clone());
                continue;
            }
            _ => {}
        }
        // Remove body if any of the following are true:
        // - function is not visible
        // - function is abstract
        // - function is opaque and not revealed
        // - function is exec or proof
        let vis = f.x.visibility.clone();
        let vis = Visibility { is_private: vis.is_private || f.x.is_abstract, ..vis };
        let is_vis = is_visible_to(&vis, module);
        let is_revealed = f.x.fuel > 0 || revealed_functions.contains(&f.x.name);
        let is_spec = f.x.mode == Mode::Spec;
        if is_vis && is_revealed && is_spec {
            functions.push(f.clone());
        } else if f.x.body.is_none() {
            functions.push(f.clone());
        } else {
            let mut function = f.x.clone();
            function.body = None;
            functions.push(Spanned::new(f.span.clone(), function));
        }
    }
    for d in &krate.datatypes {
        match &d.x.visibility.owning_module {
            Some(path) if path == module => {
                // our datatype
                state.reached_datatypes.insert(d.x.path.clone());
                state.worklist_datatypes.push(d.x.path.clone());
            }
            _ => {}
        }
        let is_vis = is_visible_to(&d.x.visibility, module);
        let is_transparent = is_datatype_transparent(module, &d);
        if is_vis {
            if is_transparent {
                datatypes.push(d.clone());
            } else {
                let mut datatype = d.x.clone();
                datatype.variants = Arc::new(vec![]);
                datatypes.push(Spanned::new(d.span.clone(), datatype));
            }
        }
    }

    let mut function_map: HashMap<Fun, Function> = HashMap::new();
    let mut datatype_map: HashMap<Path, Datatype> = HashMap::new();
    let mut all_functions_in_each_module: HashMap<Path, Vec<Fun>> = HashMap::new();
    for f in &functions {
        function_map.insert(f.x.name.clone(), f.clone());
        let module = f.x.name.path.pop_segment();
        if !all_functions_in_each_module.contains_key(&module) {
            all_functions_in_each_module.insert(module.clone(), Vec::new());
        }
        all_functions_in_each_module.get_mut(&module).unwrap().push(f.x.name.clone());
    }
    for d in &datatypes {
        datatype_map.insert(d.x.path.clone(), d.clone());
    }
    let ctxt = Ctxt { function_map, datatype_map, all_functions_in_each_module };
    traverse_reachable(&ctxt, &mut state);

    let kratex = KrateX {
        functions: functions
            .into_iter()
            .filter(|f| state.reached_functions.contains(&f.x.name))
            .collect(),
        datatypes: datatypes
            .into_iter()
            .filter(|d| state.reached_datatypes.contains(&d.x.path))
            .collect(),
        module_ids: krate.module_ids.clone(),
    };
    Arc::new(kratex)
}
