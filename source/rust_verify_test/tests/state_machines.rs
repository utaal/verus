// TODO #![feature(rustc_private)]
// TODO #[macro_use]
// TODO mod common;
// TODO use common::*;
// TODO
// TODO const IMPORTS: &str = code_str! {
// TODO     #[allow(unused_imports)] use vstd::{atomic::*};
// TODO     #[allow(unused_imports)] use vstd::{modes::*};
// TODO     #[allow(unused_imports)] use vstd::result::*;
// TODO     #[allow(unused_imports)] use vstd::option::*;
// TODO     #[allow(unused_imports)] use vstd::map::*;
// TODO     #[allow(unused_imports)] use vstd::set::*;
// TODO     #[allow(unused_imports)] use vstd::multiset::*;
// TODO     #[allow(unused_imports)] use vstd::pervasive::*;
// TODO     #[allow(unused_imports)] use builtin::*;
// TODO     #[allow(unused_imports)] use builtin_macros::*;
// TODO     #[allow(unused_imports)] use state_machines_macros::*;
// TODO
// TODO     verus!{
// TODO
// TODO     #[is_variant]
// TODO     pub ghost enum Foo {
// TODO         Bar(int),
// TODO         Qax(int),
// TODO         Duck(int),
// TODO     }
// TODO
// TODO     }
// TODO };
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] dupe_name_fail IMPORTS.to_string() + verus_code_str! {
// TODO         state_machine!{ X {
// TODO             fields {
// TODO                 pub v: Map<int, int>,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 some_name() {
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 some_name() {
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "duplicate item name")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] test_birds_eye_init_error IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields { #[sharding(variable)] pub t: int }
// TODO
// TODO             init!{
// TODO                 initialize() {
// TODO                     birds_eye let x = 5; // error
// TODO                     init t = 5;
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "`birds_eye` has no effect in an init!")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] test_birds_eye_nontokenized_error IMPORTS.to_string() + verus_code_str! {
// TODO         state_machine!{ X {
// TODO             fields { pub t: int }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     birds_eye let x = 5; // error
// TODO                     update t = 5;
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "`birds_eye` only makes sense for tokenized state machines")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] test_birds_eye_guard IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(storage_option)] pub so: Option<int>
// TODO             }
// TODO
// TODO             property!{
// TODO                 tr() {
// TODO                     birds_eye let x = 5;
// TODO                     guard so >= Some(x); // error: guard depends on birds_eye variable
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "a guard value must be a deterministic function")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] test_withdraw_bind_guard IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(storage_option)] pub so: Option<int>
// TODO             }
// TODO
// TODO             property!{
// TODO                 tr() {
// TODO                     withdraw so -= Some(let y);
// TODO                     guard so >= Some(x); // error: guard depends on withdraw binding
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "a guard value must be a deterministic function")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] test_birds_eye_req IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(storage_option)] pub so: Option<int>
// TODO             }
// TODO
// TODO             property!{
// TODO                 tr() {
// TODO                     birds_eye let x = 5;
// TODO                     require(x == 5); // error: require depends on birds_eye variable
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "'require' statements should not be in the scope of a `birds_eye` let-binding")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] require_let_birds_eye_fail IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ Y {
// TODO             fields {
// TODO                 #[sharding(variable)]
// TODO                 pub opt: Option<int>,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr1() {
// TODO                     require birds_eye let x = 5;
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "'require' statements should not be in the scope of a `birds_eye` let-binding")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] test_withdraw_bind_req IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(storage_option)] pub so: Option<int>
// TODO             }
// TODO
// TODO             property!{
// TODO                 tr() {
// TODO                     withdraw so -= Some(let x);
// TODO                     require(x == 5); // error: require depends on birds_eye variable
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "'require' statements should not be in the scope of a `withdraw` let-binding")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] test_birds_eye_req2 IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(storage_option)] pub so: Option<int>
// TODO             }
// TODO
// TODO             property!{
// TODO                 tr() {
// TODO                     if 0 == 0 {
// TODO                         birds_eye let x = 5;
// TODO                         assert(x == 5);
// TODO                     }
// TODO                     require(x == 5); // error: require depends on birds_eye variable
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "'require' statements should not be preceeded by an assert which is in the scope of")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] test_withdraw_bind_req2 IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(storage_option)] pub so: Option<int>
// TODO             }
// TODO
// TODO             property!{
// TODO                 tr() {
// TODO                     if 0 == 0 {
// TODO                         withdraw so -= Some(let x);
// TODO                         assert(x == 5);
// TODO                     }
// TODO                     require(x == 5); // error: require depends on withdraw binding
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "'require' statements should not be preceeded by an assert which is in the scope of")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] test_birds_eye_special IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(option)] pub so: Option<int>
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     birds_eye let x = 5;
// TODO                     remove so -= Some(x); // error: depends on birds_eye variable
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "'remove' statements should not be in the scope of a `birds_eye` let-binding")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] test_withdraw_binding_remove IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(option)] pub so: Option<int>
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     withdraw so -= Some(let x);
// TODO                     remove so -= Some(x); // error: depends on birds_eye variable
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "'remove' statements should not be in the scope of a `withdraw` let-binding")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] test_birds_eye_special2 IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(option)] pub so: Option<int>
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     if 0 == 0 {
// TODO                         birds_eye let x = 5;
// TODO                         assert(x == 5);
// TODO                     }
// TODO                     remove so -= Some(0); // error: depends on birds_eye variable
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "'remove' statements should not be preceeded by an assert which is in the scope of")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] test_update_constant IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(constant)] pub t: int
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     update t = 5;
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "'update' statement not allowed for field with sharding strategy 'constant'")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] test_add_constant IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(constant)] pub t: int
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     add t += (5);
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "'add' statement not allowed for field with sharding strategy 'constant'")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] test_have_constant IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(constant)] pub t: int
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     have t >= (5);
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "'have' statement not allowed for field with sharding strategy 'constant'")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] test_use_option_directly IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(option)] pub t: Option<int>,
// TODO                 #[sharding(variable)] pub v: int,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     update v = pre.t.get_Some_0();
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "cannot be directly referenced here")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] test_use_map_directly IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(map)] pub t: Map<int, int>,
// TODO                 #[sharding(variable)] pub v: int,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     update v = pre.t.index(0);
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "cannot be directly referenced here")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] test_use_multiset_directly IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(multiset)] pub t: Multiset<int>,
// TODO                 #[sharding(variable)] pub v: Multiset<int>,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     update v = pre.t;
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "cannot be directly referenced here")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] test_use_storage_option_directly IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(storage_option)] pub t: Option<int>,
// TODO                 #[sharding(variable)] pub v: int,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     update v = pre.t.get_Some_0();
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "cannot be directly referenced here")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] test_use_nottokenized_directly IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(not_tokenized)] pub t: int,
// TODO                 #[sharding(variable)] pub v: int,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     update v = pre.t;
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "cannot be directly referenced here")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] test_use_pre_no_field IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(variable)] pub v: int,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     update v = { let s = pre; s.v };
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "cannot be used opaquely")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] test_use_pre_no_field_withdraw_kv_value IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(storage_map)] pub v: Map<int, int>,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     withdraw v -= [5 => { let s = pre; s.v } ];
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "cannot be used opaquely")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] test_use_pre_no_field_remove_kv_key IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(map)] pub v: Map<int, int>,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     remove v -= [5 => { let s = pre; s.v } ];
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "cannot be used opaquely")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] test_use_pre_no_field_withdraw_kv_key IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(storage_map)] pub v: Map<int, int>,
// TODO             }
// TODO
// TODO             init!{
// TODO                 initialize() {
// TODO                     init v = Map::empty();
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     // this is ok:
// TODO                     withdraw v -= [{ let s = pre; s.v.index(0) } => 5]
// TODO                           by { assume(false); };
// TODO                 }
// TODO             }
// TODO         }}
// TODO
// TODO         verus!{
// TODO
// TODO         proof fn foo(tracked m: Map<int, int>) {
// TODO             requires(equal(m, Map::empty()));
// TODO
// TODO             let tracked inst = X::Instance::initialize(m);
// TODO             let tracked t = (inst).tr();
// TODO             assert(t === 5);
// TODO         }
// TODO
// TODO         }
// TODO     } => Ok(())
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] test_use_pre_no_field2 IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(variable)] pub v: int,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     update v = pre.some_fn();
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "`pre` cannot be used opaquely")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] test_use_pre_no_field3 IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(variable)] pub v: int,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     update v = pre.not_a_field;
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "any field access must be a state field")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] test_use_pre_no_field4 IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(variable)] pub v: int,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     update v = pre.0;
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "expected a named field")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] field_name_reserved_ident1 IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(variable)] pub instance: int,
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "reserved identifier")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] field_name_reserved_ident2 IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(variable)] pub param_token_a: int,
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "reserved identifier")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] sm_name_reserved_ident1 IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ instance {
// TODO             fields {
// TODO                 #[sharding(variable)] pub t: int,
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "reserved identifier")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] sm_name_reserved_ident2 IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ param_token_a {
// TODO             fields {
// TODO                 #[sharding(variable)] pub t: int,
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "reserved identifier")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] let_name_reserved_ident1 IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(variable)] pub t: int,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     let instance = 5;
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "reserved identifier")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] let_name_reserved_ident2 IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(variable)] pub t: int,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     let param_token_a = 5;
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "reserved identifier")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] arg_reserved_ident1 IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(variable)] pub t: int,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr(instance: int) {
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "reserved identifier")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] arg_reserved_ident2 IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(variable)] pub t: int,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr(param_token_a: int) {
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "reserved identifier")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] binding_reserved_ident1 IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(option)] pub t: Option<int>,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     remove t -= Some(let instance);
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "reserved identifier")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] duplicate_inductive_lemma IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(variable)] pub t: int,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr(x: int) {
// TODO                     update t = x;
// TODO                 }
// TODO             }
// TODO
// TODO             #[inductive(tr)]
// TODO             pub fn lemma_tr1(pre: Self, post: Self, x: int) {
// TODO             }
// TODO
// TODO             #[inductive(tr)]
// TODO             pub fn lemma_tr2(pre: Self, post: Self, x: int) {
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "duplicate 'inductive' lemma")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] missing_inductive_lemma IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(variable)] pub t: int,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr(x: int) {
// TODO                     update t = x;
// TODO                 }
// TODO             }
// TODO
// TODO             #[invariant]
// TODO             pub fn the_inv(self) -> bool {
// TODO                 self.t == 5
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "missing inductiveness proofs for")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] missing_inductive_lemma_init IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(variable)] pub t: int,
// TODO             }
// TODO
// TODO             init!{
// TODO                 tr(x: int) {
// TODO                     init t = x;
// TODO                 }
// TODO             }
// TODO
// TODO             #[invariant]
// TODO             pub fn the_inv(self) -> bool {
// TODO                 self.t == 5
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "missing inductiveness proofs for")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] inductive_lemma_readonly IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(variable)] pub t: int,
// TODO             }
// TODO
// TODO             readonly!{
// TODO                 tr(x: int) {
// TODO                 }
// TODO             }
// TODO
// TODO             #[inductive(tr)]
// TODO             pub fn lemma_tr1(pre: Self, post: Self, x: int) {
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "'inductive' lemma does not make sense for a 'readonly' transition")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] inductive_lemma_property IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(variable)] pub t: int,
// TODO             }
// TODO
// TODO             property!{
// TODO                 tr(x: int) {
// TODO                 }
// TODO             }
// TODO
// TODO             #[inductive(tr)]
// TODO             pub fn lemma_tr1(pre: Self, post: Self, x: int) {
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "'inductive' lemma does not make sense for a 'property' definition")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] lemma_wrong_args IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(variable)] pub t: int,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr(x: int) {
// TODO                 }
// TODO             }
// TODO
// TODO             #[inductive(tr)]
// TODO             pub fn lemma_tr1(pre: Self, post: Self, y: int) {
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "params for 'inductive' lemma should be")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] lemma_bad_transition_name IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(variable)] pub t: int,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr(x: int) {
// TODO                 }
// TODO             }
// TODO
// TODO             #[inductive(tro)]
// TODO             pub fn lemma_tr1(pre: Self, post: Self, x: int) {
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "could not find transition")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] lemma_bad_generic_params IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(variable)] pub t: int,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr(x: int) {
// TODO                 }
// TODO             }
// TODO
// TODO             #[inductive(tr)]
// TODO             pub fn lemma_tr1<T>(pre: Self, post: Self, x: int) {
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "should have no generic parameters")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] lemma_bad_return_type IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(variable)] pub t: int,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr(x: int) {
// TODO                 }
// TODO             }
// TODO
// TODO             #[inductive(tr)]
// TODO             pub fn lemma_tr1(pre: Self, post: Self, x: int) -> bool {
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "should have no return type")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] lemma_bad_header IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(variable)] pub t: int,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr(x: int) {
// TODO                 }
// TODO             }
// TODO
// TODO             #[inductive(tr)]
// TODO             pub fn lemma_tr1(pre: Self, post: Self, x: int) {
// TODO                 requires(true);
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "the precondition and postcondition are implicit")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] lemma_doesnt_verify IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(variable)] pub t: int,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr(x: int) {
// TODO                     update t = x;
// TODO                 }
// TODO             }
// TODO
// TODO             #[invariant]
// TODO             pub fn the_inv(self) -> bool {
// TODO                 self.t == 5
// TODO             }
// TODO
// TODO             #[inductive(tr)]
// TODO             pub fn lemma_tr1(pre: Self, post: Self, x: int) {
// TODO             } // FAILS
// TODO         }}
// TODO     } => Err(e) => assert_one_fails(e)
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] lemma_doesnt_verify_init IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(variable)] pub t: int,
// TODO             }
// TODO
// TODO             init!{
// TODO                 tr(x: int) {
// TODO                     init t = x;
// TODO                 }
// TODO             }
// TODO
// TODO             #[invariant]
// TODO             pub fn the_inv(self) -> bool {
// TODO                 self.t == 5
// TODO             }
// TODO
// TODO             #[inductive(tr)]
// TODO             pub fn lemma_tr1(post: Self, x: int) {
// TODO             } // FAILS
// TODO         }}
// TODO     } => Err(e) => assert_one_fails(e)
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] sm_generic_param_not_type IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X<'a> {
// TODO             fields {
// TODO                 #[sharding(variable)] pub t: int,
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "Only generic type parameters are supported")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] multiple_fields IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(variable)] pub t: int,
// TODO             }
// TODO
// TODO             fields {
// TODO                 #[sharding(variable)] pub x: int,
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "Expected only one declaration of `fields` block")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] no_fields IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "'fields' declaration was not found")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] conflicting_attrs IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(variable)] pub t: int,
// TODO             }
// TODO
// TODO             init!{
// TODO                 tr(x: int) {
// TODO                     init t = x;
// TODO                 }
// TODO             }
// TODO
// TODO             #[invariant]
// TODO             #[inductive(tr)]
// TODO             pub fn the_inv(self) -> bool {
// TODO                 self.t == 5
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "conflicting attributes")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] explicit_mode_inv IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(variable)] pub t: int,
// TODO             }
// TODO
// TODO             init!{
// TODO                 tr(x: int) {
// TODO                     init t = x;
// TODO                 }
// TODO             }
// TODO
// TODO             #[invariant]
// TODO             #[verifier::spec]
// TODO             pub fn the_inv(self) -> bool {
// TODO                 true
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "should not be explicitly labelled")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] wrong_mode_inv IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(variable)] pub t: int,
// TODO             }
// TODO
// TODO             init!{
// TODO                 tr(x: int) {
// TODO                     init t = x;
// TODO                 }
// TODO             }
// TODO
// TODO             #[invariant]
// TODO             pub proof fn the_inv(self) -> bool {
// TODO                 true
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "an invariant function should be `spec`")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] wrong_mode_inductive IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(variable)] pub t: int,
// TODO             }
// TODO
// TODO             init!{
// TODO                 tr(x: int) {
// TODO                     init t = x;
// TODO                 }
// TODO             }
// TODO
// TODO             #[invariant]
// TODO             pub fn the_inv(self) -> bool {
// TODO                 true
// TODO             }
// TODO
// TODO             #[inductive(tr)]
// TODO             pub spec fn lemma_tr1(post: Self, x: int) {
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "an inductiveness lemma should be `proof`")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] explicit_mode_field IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(variable)] #[verifier::spec] pub t: int,
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "should not be explicitly labelled")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] explicit_mode_proof IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(variable)] pub t: int,
// TODO             }
// TODO
// TODO             init!{
// TODO                 tr(x: int) {
// TODO                     init t = x;
// TODO                 }
// TODO             }
// TODO
// TODO             #[invariant]
// TODO             pub fn the_inv(self) -> bool {
// TODO                 true
// TODO             }
// TODO
// TODO             #[inductive(tr)]
// TODO             #[verifier::proof]
// TODO             pub fn lemma_tr1(post: Self, x: int) {
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "should not be explicitly labelled")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] inv_wrong_params IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(variable)] pub t: int,
// TODO             }
// TODO
// TODO             init!{
// TODO                 tr(x: int) {
// TODO                     init t = x;
// TODO                 }
// TODO             }
// TODO
// TODO             #[invariant]
// TODO             pub fn the_inv(x: int) -> bool {
// TODO                 true
// TODO             }
// TODO
// TODO             #[inductive(tr)]
// TODO             #[verifier::proof]
// TODO             pub fn lemma_tr1(post: Self, x: int) {
// TODO             } // FAILS
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "an invariant function must take 1 argument (self)")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] inv_wrong_ret IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(variable)] pub t: int,
// TODO             }
// TODO
// TODO             init!{
// TODO                 tr(x: int) {
// TODO                     init t = x;
// TODO                 }
// TODO             }
// TODO
// TODO             #[invariant]
// TODO             pub fn the_inv(self) -> int {
// TODO                 5
// TODO             }
// TODO
// TODO             #[inductive(tr)]
// TODO             #[verifier::proof]
// TODO             pub fn lemma_tr1(post: Self, x: int) {
// TODO             } // FAILS
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "an invariant function must return a bool")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] inv_wrong_generics IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(variable)] pub t: int,
// TODO             }
// TODO
// TODO             init!{
// TODO                 tr(x: int) {
// TODO                     init t = x;
// TODO                 }
// TODO             }
// TODO
// TODO             #[invariant]
// TODO             pub fn the_inv<V>(self) -> bool {
// TODO                 true
// TODO             }
// TODO
// TODO             #[inductive(tr)]
// TODO             #[verifier::proof]
// TODO             pub fn lemma_tr1(post: Self, x: int) {
// TODO             } // FAILS
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "an invariant function must take 0 type arguments")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] normal_sm_sharding IMPORTS.to_string() + verus_code_str! {
// TODO         state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(variable)] pub t: int,
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "sharding strategy only makes sense for tokenized state machines")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] tokenized_sm_no_sharding IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 pub t: int,
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "tokenized state machine requires a sharding strategy")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] unrecognized_sharding_strategy_name IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(foo)] pub t: int,
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "unrecognized sharding strategy")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] duplicate_sharding_attr IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(variable)]
// TODO                 #[sharding(variable)]
// TODO                 pub t: int,
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "duplicate sharding attribute")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] wrong_form_option IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(option)]
// TODO                 pub t: int,
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "must be of the form Option<_>")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] wrong_form_option2 IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(option)]
// TODO                 pub t: Multiset<int>,
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "must be of the form Option<_>")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] wrong_form_option3 IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(option)]
// TODO                 pub t: Map<int, int>,
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "must be of the form Option<_>")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] wrong_form_storage_option IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(storage_option)]
// TODO                 pub t: int,
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "must be of the form Option<_>")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] wrong_form_map IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(map)]
// TODO                 pub t: int,
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "must be of the form Map<_, _>")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] wrong_form_storage_map IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(storage_map)]
// TODO                 pub t: int,
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "must be of the form Map<_, _>")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] wrong_form_multiset IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(multiset)]
// TODO                 pub t: int,
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "must be of the form Multiset<_>")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] wrong_form_set IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(set)]
// TODO                 pub t: Multiset<int>,
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "must be of the form Set<_>")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] wrong_form_count IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(count)]
// TODO                 pub t: int,
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "must be nat")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] wrong_form_bool IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(bool)]
// TODO                 pub t: int,
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "must be bool")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] special_op_conditional IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(option)]
// TODO                 pub t: Option<int>,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     if true {
// TODO                         add t += Some(5);
// TODO                     }
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "statements are not supported inside conditionals")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] special_op_binding_conditional IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(option)]
// TODO                 pub t: Option<int>,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     if true {
// TODO                         remove t -= Some(let x);
// TODO                     }
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "statements are not supported inside conditionals")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] special_op_match IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(option)]
// TODO                 pub t: Option<int>,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr(foo: Foo) {
// TODO                     match foo {
// TODO                         Foo::Bar(x) => {
// TODO                             add t += Some(5);
// TODO                         }
// TODO                         Foo::Qax(y) => { }
// TODO                         Foo::Duck(z) => { }
// TODO                     }
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "statements are not supported inside conditionals")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] remove_after_have IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(option)]
// TODO                 pub t: Option<int>,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     have t >= Some(5);
// TODO                     remove t -= Some(5);
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "remove -> have -> add")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] remove_after_have_with_binding IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(option)]
// TODO                 pub t: Option<int>,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     have t >= Some(let z);
// TODO                     remove t -= Some(let x);
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "remove -> have -> add")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] have_after_add IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(option)]
// TODO                 pub t: Option<int>,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     add t += Some(5);
// TODO                     have t >= Some(5);
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "remove -> have -> add")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] remove_after_add IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(option)]
// TODO                 pub t: Option<int>,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     add t += Some(5);
// TODO                     remove t -= Some(5);
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "remove -> have -> add")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] init_wf_init_missing IMPORTS.to_string() + verus_code_str! {
// TODO         state_machine!{ X {
// TODO             fields {
// TODO                 pub t: int,
// TODO             }
// TODO
// TODO             init!{
// TODO                 init() {
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "procedure does not initialize")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] init_wf_init_dupe IMPORTS.to_string() + verus_code_str! {
// TODO         state_machine!{ X {
// TODO             fields {
// TODO                 pub t: int,
// TODO             }
// TODO
// TODO             init!{
// TODO                 init() {
// TODO                     init t = 5;
// TODO                     init t = 6;
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "might be initialized multiple times")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] init_wf_init_dupe_conditional IMPORTS.to_string() + verus_code_str! {
// TODO         state_machine!{ X {
// TODO             fields {
// TODO                 pub t: int,
// TODO             }
// TODO
// TODO             init!{
// TODO                 init() {
// TODO                     init t = 5;
// TODO                     if 1 + 2 == 3 {
// TODO                         init t = 6;
// TODO                     } else {
// TODO                         init t = 7;
// TODO                     }
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "might be initialized multiple times")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] init_wf_init_if IMPORTS.to_string() + verus_code_str! {
// TODO         state_machine!{ X {
// TODO             fields {
// TODO                 pub t: int,
// TODO             }
// TODO
// TODO             init!{
// TODO                 init() {
// TODO                     if 1 + 2 == 3 {
// TODO                         init t = 6;
// TODO                     } else {
// TODO                     }
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "the else-branch does not initialize")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] init_wf_init_dupe_match IMPORTS.to_string() + verus_code_str! {
// TODO         state_machine!{ X {
// TODO             fields {
// TODO                 pub t: int,
// TODO             }
// TODO
// TODO             init!{
// TODO                 init(foo: Foo) {
// TODO                     init t = 5;
// TODO                     match foo {
// TODO                         Foo::Bar(x) => { init t = 6; }
// TODO                         Foo::Qax(y) => { init t = 7; }
// TODO                         Foo::Duck(z) => { init t = 8; }
// TODO                     }
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "might be initialized multiple times")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] init_wf_init_else IMPORTS.to_string() + verus_code_str! {
// TODO         state_machine!{ X {
// TODO             fields {
// TODO                 pub t: int,
// TODO             }
// TODO
// TODO             init!{
// TODO                 init() {
// TODO                     if 1 + 2 == 3 {
// TODO                     } else {
// TODO                         init t = 6;
// TODO                     }
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "the if-branch does not initialize")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] init_wf_init_match IMPORTS.to_string() + verus_code_str! {
// TODO         state_machine!{ X {
// TODO             fields {
// TODO                 pub t: int,
// TODO             }
// TODO
// TODO             init!{
// TODO                 init() {
// TODO                     match foo {
// TODO                         Foo::Bar(x) => {
// TODO                             init t = 6;
// TODO                         }
// TODO                         Foo::Qax(y) => {
// TODO                         }
// TODO                         Foo::Duck(z) => {
// TODO                             init t = 7;
// TODO                         }
// TODO                     }
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "all branches of a match-statement must initialize")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] init_wf_init_match2 IMPORTS.to_string() + verus_code_str! {
// TODO         state_machine!{ X {
// TODO             fields {
// TODO                 pub t: int,
// TODO             }
// TODO
// TODO             init!{
// TODO                 init() {
// TODO                     match foo {
// TODO                         Foo::Bar(x) => {
// TODO                         }
// TODO                         Foo::Qax(y) => {
// TODO                             init t = 6;
// TODO                         }
// TODO                         Foo::Duck(z) => {
// TODO                         }
// TODO                     }
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "all branches of a match-statement must initialize")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] init_wf_update IMPORTS.to_string() + verus_code_str! {
// TODO         state_machine!{ X {
// TODO             fields {
// TODO                 pub t: int,
// TODO             }
// TODO
// TODO             init!{
// TODO                 init() {
// TODO                     init t = 6;
// TODO                     update t = 5;
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "'update' statement not allowed in initialization")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] init_wf_update2 IMPORTS.to_string() + verus_code_str! {
// TODO         state_machine!{ X {
// TODO             fields {
// TODO                 pub t: int,
// TODO             }
// TODO
// TODO             init!{
// TODO                 init() {
// TODO                     update t = 5;
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "'update' statement not allowed in initialization")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] init_wf_special IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(option)]
// TODO                 pub t: Option<int>,
// TODO             }
// TODO
// TODO             init!{
// TODO                 init() {
// TODO                     add t += Some(5);
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "use 'init' instead")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] init_wf_special_with_binding IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(option)]
// TODO                 pub t: Option<int>,
// TODO             }
// TODO
// TODO             init!{
// TODO                 init() {
// TODO                     remove t -= Some(let x);
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "use 'init' instead")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] init_wf_assert IMPORTS.to_string() + verus_code_str! {
// TODO         state_machine!{ X {
// TODO             fields {
// TODO                 pub t: int,
// TODO             }
// TODO
// TODO             init!{
// TODO                 init() {
// TODO                     assert(true);
// TODO                     init t = 6;
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "'assert' statement not allowed in initialization")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] normal_wf_update_dupe IMPORTS.to_string() + verus_code_str! {
// TODO         state_machine!{ X {
// TODO             fields {
// TODO                 pub t: int,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     update t = 5;
// TODO                     update t = 6;
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "might be updated multiple times")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] normal_wf_update_dupe_conditional IMPORTS.to_string() + verus_code_str! {
// TODO         state_machine!{ X {
// TODO             fields {
// TODO                 pub t: int,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     update t = 5;
// TODO                     if true {
// TODO                         update t = 6;
// TODO                     }
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "might be updated multiple times")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] normal_wf_update_dupe_conditional2 IMPORTS.to_string() + verus_code_str! {
// TODO         state_machine!{ X {
// TODO             fields {
// TODO                 pub t: int,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     update t = 5;
// TODO                     if true {
// TODO                     } else {
// TODO                         update t = 6;
// TODO                     }
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "might be updated multiple times")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] normal_wf_update_dupe_match IMPORTS.to_string() + verus_code_str! {
// TODO         state_machine!{ X {
// TODO             fields {
// TODO                 pub t: int,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     update t = 5;
// TODO                     match foo {
// TODO                         Foo::Bar(x) => {
// TODO                             update t = 6;
// TODO                         }
// TODO                         Foo::Qax(y) => { }
// TODO                         Foo::Duck(z) => { }
// TODO                     }
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "might be updated multiple times")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] normal_wf_update_init IMPORTS.to_string() + verus_code_str! {
// TODO         state_machine!{ X {
// TODO             fields {
// TODO                 pub t: int,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     init t = 5;
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "'init' statement not allowed")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] normal_wf_update_guard IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(storage_option)]
// TODO                 pub t: Option<int>,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     guard t >= Some(5);
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "'guard' statement only allowed in 'readonly' transition or 'property' definition")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] readonly_wf_update IMPORTS.to_string() + verus_code_str! {
// TODO         state_machine!{ X {
// TODO             fields {
// TODO                 pub t: int,
// TODO             }
// TODO
// TODO             readonly!{
// TODO                 tr() {
// TODO                     update t = 5;
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "statement not allowed in readonly transition")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] property_wf_update IMPORTS.to_string() + verus_code_str! {
// TODO         state_machine!{ X {
// TODO             fields {
// TODO                 pub t: int,
// TODO             }
// TODO
// TODO             property!{
// TODO                 tr() {
// TODO                     update t = 5;
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "statement not allowed in property definition")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] readonly_wf_init IMPORTS.to_string() + verus_code_str! {
// TODO         state_machine!{ X {
// TODO             fields {
// TODO                 pub t: int,
// TODO             }
// TODO
// TODO             readonly!{
// TODO                 tr() {
// TODO                     init t = 5;
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "statement not allowed outside 'init' routine")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] property_wf_init IMPORTS.to_string() + verus_code_str! {
// TODO         state_machine!{ X {
// TODO             fields {
// TODO                 pub t: int,
// TODO             }
// TODO
// TODO             property!{
// TODO                 tr() {
// TODO                     init t = 5;
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "statement not allowed outside 'init' routine")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] readonly_wf_add IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(option)]
// TODO                 pub t: Option<int>,
// TODO             }
// TODO
// TODO             readonly!{
// TODO                 tr() {
// TODO                     add t += Some(5);
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "statement not allowed in readonly transition")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] property_wf_add IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(option)]
// TODO                 pub t: Option<int>,
// TODO             }
// TODO
// TODO             property!{
// TODO                 tr() {
// TODO                     add t += Some(5);
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "statement not allowed in 'property' definition")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] readonly_wf_remove_with_binding IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(option)]
// TODO                 pub t: Option<int>,
// TODO             }
// TODO
// TODO             readonly!{
// TODO                 tr() {
// TODO                     remove t -= Some(let x);
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "statement not allowed in readonly transition")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] readonly_wf_remove IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(option)]
// TODO                 pub t: Option<int>,
// TODO             }
// TODO
// TODO             readonly!{
// TODO                 tr() {
// TODO                     remove t -= Some(5);
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "statement not allowed in readonly transition")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] readonly_wf_deposit IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(storage_option)]
// TODO                 pub t: Option<int>,
// TODO             }
// TODO
// TODO             readonly!{
// TODO                 tr() {
// TODO                     deposit t += Some(5);
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "statement not allowed in readonly transition")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] readonly_wf_withdraw IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(storage_option)]
// TODO                 pub t: Option<int>,
// TODO             }
// TODO
// TODO             readonly!{
// TODO                 tr() {
// TODO                     withdraw t -= Some(5);
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "statement not allowed in readonly transition")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] field_not_found IMPORTS.to_string() + verus_code_str! {
// TODO         state_machine!{ X {
// TODO             fields {
// TODO                 pub t: int,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     update whats_this = 5;
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "field 'whats_this' not found")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] inherent_safety_condition_option_remove IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(option)]
// TODO                 pub t: Option<int>
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     remove t -= Some(5) by { };
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "adding a proof body is meaningless")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] inherent_safety_condition_option_remove_with_binding IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(option)]
// TODO                 pub t: Option<int>
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     remove t -= Some(let y) by { };
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "adding a proof body is meaningless")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] inherent_safety_condition_map_remove IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(map)]
// TODO                 pub t: Map<int, int>
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     remove t -= [5 => 7] by { };
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "adding a proof body is meaningless")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] inherent_safety_condition_multiset_remove IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(multiset)]
// TODO                 pub t: Multiset<int>
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     remove t -= { 5 } by { };
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "adding a proof body is meaningless")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] inherent_safety_condition_option_add IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(option)]
// TODO                 pub t: Option<int>
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     add t += Some(5) by { }; // FAILS
// TODO                 }
// TODO             }
// TODO
// TODO             #[inductive(tr)]
// TODO             pub fn is_inductive(pre: Self, post: Self) {
// TODO                 assert(pre.t.is_None());
// TODO                 assert(post.t.is_Some());
// TODO                 assert(post.t.get_Some_0() == 5);
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_one_fails(e)
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] inherent_safety_condition_option_general_add IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(option)]
// TODO                 pub t: Option<int>
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     add t += (Option::Some(5)) by { }; // FAILS
// TODO                 }
// TODO             }
// TODO
// TODO             #[inductive(tr)]
// TODO             pub fn is_inductive(pre: Self, post: Self) {
// TODO                 assert(pre.t.is_None());
// TODO                 assert(post.t.is_Some());
// TODO                 assert(post.t.get_Some_0() == 5);
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_one_fails(e)
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] inherent_safety_condition_map_add IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(map)]
// TODO                 pub t: Map<int, int>
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     add t += [5 => 7] by { }; // FAILS
// TODO                 }
// TODO             }
// TODO
// TODO             #[inductive(tr)]
// TODO             pub fn is_inductive(pre: Self, post: Self) {
// TODO                 assert(!pre.t.dom().contains(5));
// TODO                 assert(post.t.dom().contains(5));
// TODO                 assert(post.t.index(5) == 7);
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_one_fails(e)
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] inherent_safety_condition_map_general_add IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(map)]
// TODO                 pub t: Map<int, int>
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     add t += (Map::<int, int>::empty().insert(5, 7)) by { }; // FAILS
// TODO                 }
// TODO             }
// TODO
// TODO             #[inductive(tr)]
// TODO             pub fn is_inductive(pre: Self, post: Self) {
// TODO                 assert(!pre.t.dom().contains(5));
// TODO                 assert(post.t.dom().contains(5));
// TODO                 assert(post.t.index(5) == 7);
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_one_fails(e)
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] inherent_safety_condition_multiset_add IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(multiset)]
// TODO                 pub t: Multiset<int>
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     add t += { 5 } by { };
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "adding a proof body is meaningless")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] inherent_safety_condition_multiset_general_add IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(multiset)]
// TODO                 pub t: Multiset<int>
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     add t += ({ 5 }) by { };
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "adding a proof body is meaningless")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] inherent_safety_condition_option_have IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(option)]
// TODO                 pub t: Option<int>
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     have t >= Some(5) by { };
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "adding a proof body is meaningless")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] inherent_safety_condition_map_have IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(map)]
// TODO                 pub t: Map<int, int>
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     have t >= [5 => 7] by { };
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "adding a proof body is meaningless")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] inherent_safety_condition_multiset_have IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(multiset)]
// TODO                 pub t: Multiset<int>
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     have t >= { 5 } by { };
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "adding a proof body is meaningless")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] inherent_safety_condition_option_withdraw IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(storage_option)]
// TODO                 pub t: Option<int>
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     withdraw t -= Some(5) by { }; // FAILS
// TODO                 }
// TODO             }
// TODO
// TODO             #[inductive(tr)]
// TODO             pub fn is_inductive(pre: Self, post: Self) {
// TODO                 assert(pre.t.is_Some());
// TODO                 assert(pre.t.get_Some_0() == 5);
// TODO                 assert(post.t.is_None());
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_one_fails(e)
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] inherent_safety_condition_map_withdraw IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(storage_map)]
// TODO                 pub t: Map<int, int>
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     withdraw t -= [5 => 7] by { }; // FAILS
// TODO                 }
// TODO             }
// TODO
// TODO             #[inductive(tr)]
// TODO             pub fn is_inductive(pre: Self, post: Self) {
// TODO                 assert(pre.t.dom().contains(5));
// TODO                 assert(pre.t.index(5) == 7);
// TODO                 assert(!post.t.dom().contains(5));
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_one_fails(e)
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] inherent_safety_condition_map_withdraw_with_binding IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(storage_map)]
// TODO                 pub t: Map<int, int>
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     withdraw t -= [5 => let z] by { }; // FAILS
// TODO                 }
// TODO             }
// TODO
// TODO             #[inductive(tr)]
// TODO             pub fn is_inductive(pre: Self, post: Self) {
// TODO                 assert(pre.t.dom().contains(5));
// TODO                 assert(!post.t.dom().contains(5));
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_one_fails(e)
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] inherent_safety_condition_multiset_withdraw IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(storage_multiset)]
// TODO                 pub t: Multiset<int>
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     withdraw t -= { 5 } by { }; // FAILS
// TODO                 }
// TODO             }
// TODO
// TODO             #[inductive(tr)]
// TODO             pub fn is_inductive(pre: Self, post: Self) {
// TODO                 assert(pre.t.count(5) >= 1);
// TODO                 assert(equal(post.t, pre.t.remove(5)));
// TODO             }
// TODO         }}
// TODO     // not supported right now:
// TODO     } => Err(e) => assert_error_msg(e, "storage_multiset strategy not implemented")
// TODO     //} => Err(e) => assert_one_fails(e)
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] inherent_safety_condition_option_guard IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(storage_option)]
// TODO                 pub t: Option<int>
// TODO             }
// TODO
// TODO             property!{
// TODO                 tr() {
// TODO                     guard t >= Some(5) by { }; // FAILS
// TODO
// TODO                     birds_eye let t = pre.t;
// TODO                     assert(t.is_Some() && t.get_Some_0() == 5);
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_one_fails(e)
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] inherent_safety_condition_map_guard IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(storage_map)]
// TODO                 pub t: Map<int, int>
// TODO             }
// TODO
// TODO             property!{
// TODO                 tr() {
// TODO                     guard t >= [5 => 7] by { }; // FAILS
// TODO
// TODO                     birds_eye let t = pre.t;
// TODO                     assert(t.dom().contains(5) && t.index(5) == 7);
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_one_fails(e)
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] inherent_safety_condition_option_general_guard IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(storage_option)]
// TODO                 pub t: Option<int>
// TODO             }
// TODO
// TODO             property!{
// TODO                 tr() {
// TODO                     guard t >= (Option::Some(5)) by { }; // FAILS
// TODO
// TODO                     birds_eye let t = pre.t;
// TODO                     assert(t.is_Some() && t.get_Some_0() == 5);
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_one_fails(e)
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] inherent_safety_condition_map_general_guard IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(storage_map)]
// TODO                 pub t: Map<int, int>
// TODO             }
// TODO
// TODO             property!{
// TODO                 tr() {
// TODO                     guard t >= (Map::<int,int>::empty().insert(5, 7)) by { }; // FAILS
// TODO
// TODO                     birds_eye let t = pre.t;
// TODO                     assert(t.dom().contains(5) && t.index(5) == 7) by {
// TODO                         assert(Map::<int,int>::empty().insert(5, 7).dom().contains(5));
// TODO                         assert(Map::<int,int>::empty().insert(5, 7).index(5) == 7);
// TODO                     };
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_one_fails(e)
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] inherent_safety_condition_multiset_guard IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(storage_multiset)]
// TODO                 pub t: Multiset<int>
// TODO             }
// TODO
// TODO             property!{
// TODO                 tr() {
// TODO                     guard t >= { 5 } by { }; // FAILS
// TODO
// TODO                     birds_eye let t = pre.t;
// TODO                     assert(t.count(5) >= 1);
// TODO                 }
// TODO             }
// TODO         }}
// TODO     // not supported right now:
// TODO     } => Err(e) => assert_error_msg(e, "storage_multiset strategy not implemented")
// TODO     //} => Err(e) => assert_one_fails(e)
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] inherent_safety_condition_multiset_general_guard IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(storage_multiset)]
// TODO                 pub t: Multiset<int>
// TODO             }
// TODO
// TODO             property!{
// TODO                 tr() {
// TODO                     guard t >= (Multiset::singleton(5)) by { }; // FAILS
// TODO
// TODO                     birds_eye let t = pre.t;
// TODO                     assert(t.count(5) >= 1);
// TODO                 }
// TODO             }
// TODO         }}
// TODO     // not supported right now:
// TODO     } => Err(e) => assert_error_msg(e, "unrecognized sharding strategy: 'storage_multiset'")
// TODO     //} => Err(e) => assert_one_fails(e)
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] inherent_safety_condition_option_deposit IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(storage_option)]
// TODO                 pub t: Option<int>
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     deposit t += Some(5) by { }; // FAILS
// TODO                 }
// TODO             }
// TODO
// TODO             #[inductive(tr)]
// TODO             pub fn is_inductive(pre: Self, post: Self) {
// TODO                 assert(pre.t.is_None());
// TODO                 assert(post.t.is_Some());
// TODO                 assert(post.t.get_Some_0() == 5);
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_one_fails(e)
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] inherent_safety_condition_map_deposit IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(storage_map)]
// TODO                 pub t: Map<int, int>
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     deposit t += [5 => 7] by { }; // FAILS
// TODO                 }
// TODO             }
// TODO
// TODO             #[inductive(tr)]
// TODO             pub fn is_inductive(pre: Self, post: Self) {
// TODO                 assert(!pre.t.dom().contains(5));
// TODO                 assert(post.t.dom().contains(5));
// TODO                 assert(post.t.index(5) == 7);
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_one_fails(e)
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] inherent_safety_condition_multiset_deposit IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(storage_multiset)]
// TODO                 pub t: Multiset<int>
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     deposit t += { 5 } by { };
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "storage_multiset strategy not implemented")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] assert_safety_condition_fail IMPORTS.to_string() + verus_code_str! {
// TODO         state_machine!{ X {
// TODO             fields {
// TODO                 pub t: int,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     assert(pre.t == 0); // FAILS
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_one_fails(e)
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] assert_safety_condition_readonly_fail IMPORTS.to_string() + verus_code_str! {
// TODO         state_machine!{ X {
// TODO             fields {
// TODO                 pub t: int,
// TODO             }
// TODO
// TODO             readonly!{
// TODO                 tr() {
// TODO                     assert(pre.t == 0); // FAILS
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_one_fails(e)
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] assert_safety_condition_property_fail IMPORTS.to_string() + verus_code_str! {
// TODO         state_machine!{ X {
// TODO             fields {
// TODO                 pub t: int,
// TODO             }
// TODO
// TODO             property!{
// TODO                 tr() {
// TODO                     assert(pre.t == 0); // FAILS
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_one_fails(e)
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] wrong_op_var_add_option IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(variable)]
// TODO                 pub t: Option<int>,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     add t += Some(5);
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "statement not allowed for field with sharding strategy")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] wrong_op_multiset_add_option IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(multiset)]
// TODO                 pub t: Multiset<int>,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     add t += Some(5);
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "element but the given field has sharding strategy 'multiset'")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] wrong_op_multiset_add_set IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(multiset)]
// TODO                 pub t: Multiset<int>,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     add t += set { 5 };
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "element but the given field has sharding strategy 'multiset'")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] wrong_op_set_add_multiset IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(set)]
// TODO                 pub t: Set<int>,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     add t += { 5 };
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "element but the given field has sharding strategy 'set'")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] wrong_op_multiset_add_option_with_binding IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(multiset)]
// TODO                 pub t: Multiset<int>,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     add t += Some(let z);
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "element but the given field has sharding strategy 'multiset'")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] wrong_op_map_add_option IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(map)]
// TODO                 pub t: Map<int, int>,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     add t += Some(5);
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "element but the given field has sharding strategy 'map'")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] wrong_op_option_add_map IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(option)]
// TODO                 pub t: Option<int>
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     add t += [5 => 5];
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "element but the given field has sharding strategy 'option'")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] wrong_op_option_add_multiset IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(option)]
// TODO                 pub t: Option<int>
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     add t += {5};
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "element but the given field has sharding strategy 'option'")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] wrong_op_map_add_multiset IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(map)]
// TODO                 pub t: Map<int, int>
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     add t += {5};
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "element but the given field has sharding strategy 'map'")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] wrong_op_multiset_add_map IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(multiset)]
// TODO                 pub t: Multiset<int>,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     add t += [5 => 5];
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "element but the given field has sharding strategy 'multiset'")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] wrong_op_map_guard_option IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(map)]
// TODO                 pub t: Map<int, int>
// TODO             }
// TODO
// TODO             property!{
// TODO                 tr() {
// TODO                     guard t >= Some(5);
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "element but the given field has sharding strategy 'map'")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] wrong_op_count_add_option IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(count)]
// TODO                 pub t: nat,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     add t += Some(spec_literal_nat("5"));
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "element but the given field has sharding strategy 'count'")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] wrong_op_option_deposit_option IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(option)]
// TODO                 pub t: Option<int>,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                    deposit t += Some(5);
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "is only for storage types")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] wrong_op_storage_option_add_option IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(storage_option)]
// TODO                 pub t: Option<int>,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                    add t += Some(5);
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "use deposit/withdraw/guard statements for storage strategies")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] no_let_repeated_idents IMPORTS.to_string() + verus_code_str! {
// TODO         state_machine!{ X {
// TODO             fields {
// TODO                 pub t: Map<int, int>
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     if true {
// TODO                         let x = 5;
// TODO                     } else {
// TODO                         let x = 5;
// TODO                     }
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "bound variables with the same name")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] no_let_repeated_idents2 IMPORTS.to_string() + verus_code_str! {
// TODO         state_machine!{ X {
// TODO             fields {
// TODO                 pub t: Map<int, int>
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     let x = 5;
// TODO                     let x = 5;
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "bound variables with the same name")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] no_let_repeated_idents3 IMPORTS.to_string() + verus_code_str! {
// TODO         state_machine!{ X {
// TODO             fields {
// TODO                 pub t: Map<int, int>
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr(x: int) {
// TODO                     let x = 5;
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "bound variables with the same name")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] no_let_repeated_idents4 IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(option)]
// TODO                 pub t: Option<int>,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr(x: int) {
// TODO                     remove t -= Some(let x);
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "bound variables with the same name")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] type_recursion_fail IMPORTS.to_string() + code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(variable)]
// TODO                 pub t: X::Instance,
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "recursive type")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] type_recursion_fail_negative IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 // this should fail because Map has a maybe_negative first param
// TODO
// TODO                 #[sharding(variable)]
// TODO                 pub t: Map<X::Instance, int>
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_vir_error_msg(e, "non-positive polarity")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] lemma_recursion_fail IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(variable)]
// TODO                 pub t: int,
// TODO             }
// TODO
// TODO             init!{
// TODO                 initialize() {
// TODO                     init t = 1;
// TODO                 }
// TODO             }
// TODO
// TODO             property!{
// TODO                 ro() {
// TODO                     assert(pre.t == 2);
// TODO                 }
// TODO             }
// TODO
// TODO             #[invariant]
// TODO             pub fn inv_2(self) -> bool {
// TODO                 self.t == 2
// TODO             }
// TODO
// TODO             #[inductive(initialize)]
// TODO             fn inductive_init(post: Self) {
// TODO                 let tracked (Tracked(inst), Tracked(token)) = X::Instance::initialize();
// TODO                 inst.ro(&token);
// TODO                 // this should derive a contradiction if not for the recursion checking
// TODO             }
// TODO         }}
// TODO     } => Err(err) => assert_vir_error_msg(err, "recursive function must have a decreases clause")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] lemma_recursion_assert_fail IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(variable)]
// TODO                 pub t: int,
// TODO             }
// TODO
// TODO             init!{
// TODO                 initialize() {
// TODO                     init t = 1;
// TODO                 }
// TODO             }
// TODO
// TODO             property!{
// TODO                 ro() {
// TODO                     assert(pre.t == 2) by {
// TODO                         foo_lemma();
// TODO                     };
// TODO                 }
// TODO             }
// TODO         }}
// TODO
// TODO         verus!{
// TODO         proof fn foo_lemma() {
// TODO             let tracked (Tracked(inst), Tracked(token)) = X::Instance::initialize();
// TODO             inst.ro(&token);
// TODO         }
// TODO         }
// TODO     } => Err(err) => assert_vir_error_msg(err, "recursive function must have a decreases clause")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] relation_codegen IMPORTS.to_string() + verus_code_str! {
// TODO         state_machine!{ X {
// TODO             fields {
// TODO                 pub x: int,
// TODO                 pub y: int,
// TODO                 pub z: int,
// TODO             }
// TODO
// TODO             init!{
// TODO                 initialize(x: int, y: int, z: int) {
// TODO                     init x = x;
// TODO                     init y = y;
// TODO                     require(y <= z);
// TODO                     if x == y {
// TODO                         init z = z;
// TODO                     } else {
// TODO                         init z = z + 1;
// TODO                     }
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr1(b: bool, c: bool) {
// TODO                     require(b);
// TODO                     assert(pre.y <= pre.z);
// TODO                     require(c);
// TODO                     update z = pre.z + 1;
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr2(b: bool, c: bool) {
// TODO                     if b {
// TODO                         update z = pre.z + 1;
// TODO                     } else {
// TODO                         assert(pre.y <= pre.z);
// TODO                     }
// TODO                     require(c);
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr3(b: bool, c: bool) {
// TODO                     if b {
// TODO                         assert(pre.y <= pre.z);
// TODO                     } else {
// TODO                         let j = pre.z + 1;
// TODO                         update z = j;
// TODO                     }
// TODO                     require(c);
// TODO                 }
// TODO             }
// TODO
// TODO             #[invariant]
// TODO             pub fn the_inv(self) -> bool { self.y <= self.z }
// TODO
// TODO             #[inductive(initialize)]
// TODO             fn init_inductive(post: Self, x: int, y: int, z: int) { }
// TODO
// TODO             #[inductive(tr1)]
// TODO             fn tr1_inductive(pre: Self, post: Self, b: bool, c: bool) { }
// TODO
// TODO             #[inductive(tr2)]
// TODO             fn tr2_inductive(pre: Self, post: Self, b: bool, c: bool) { }
// TODO
// TODO             #[inductive(tr3)]
// TODO             fn tr3_inductive(pre: Self, post: Self, b: bool, c: bool) { }
// TODO
// TODO         }}
// TODO
// TODO         verus! {
// TODO
// TODO         spec fn rel_init(post: X::State, x: int, y: int, z: int) -> bool {
// TODO             post.x == x && post.y == y && y <= z &&
// TODO             if x == y { post.z == z } else { post.z == z + 1 }
// TODO         }
// TODO
// TODO         spec fn rel_tr1(pre: X::State, post: X::State, b: bool, c: bool) -> bool {
// TODO             &&& b
// TODO             &&& pre.y <= pre.z ==> {
// TODO                 &&& c
// TODO                 &&& post.z == pre.z + 1
// TODO                 &&& post.x == pre.x
// TODO                 &&& post.y == pre.y
// TODO             }
// TODO         }
// TODO
// TODO         spec fn rel_tr1_strong(pre: X::State, post: X::State, b: bool, c: bool) -> bool {
// TODO             &&& b
// TODO             &&& pre.y <= pre.z && {
// TODO                 &&& c
// TODO                 &&& post.z == pre.z + 1
// TODO                 &&& post.x == pre.x
// TODO                 &&& post.y == pre.y
// TODO             }
// TODO         }
// TODO
// TODO         spec fn rel_tr2(pre: X::State, post: X::State, b: bool, c: bool) -> bool {
// TODO             &&& (if b { post.z == pre.z + 1 } else { pre.y <= pre.z ==> post.z == pre.z })
// TODO             &&& (!b ==> pre.y <= pre.z) ==> {
// TODO                 &&& c
// TODO                 &&& pre.x == post.x
// TODO                 &&& pre.y == post.y
// TODO             }
// TODO         }
// TODO
// TODO         spec fn rel_tr2_strong(pre: X::State, post: X::State, b: bool, c: bool) -> bool {
// TODO             &&& (if b { post.z == pre.z + 1 } else { post.z == pre.z })
// TODO             &&& (!b ==> pre.y <= pre.z) && {
// TODO                 &&& c
// TODO                 &&& pre.x == post.x
// TODO                 &&& pre.y == post.y
// TODO             }
// TODO         }
// TODO
// TODO         spec fn rel_tr3(pre: X::State, post: X::State, b: bool, c: bool) -> bool {
// TODO             &&& (if !b { post.z == pre.z + 1 } else { pre.y <= pre.z ==> post.z == pre.z })
// TODO             &&& (b ==> pre.y <= pre.z) ==> {
// TODO                 &&& c
// TODO                 &&& pre.x == post.x
// TODO                 &&& pre.y == post.y
// TODO             }
// TODO         }
// TODO
// TODO         spec fn rel_tr3_strong(pre: X::State, post: X::State, b: bool, c: bool) -> bool {
// TODO             &&& (if !b { post.z == pre.z + 1 } else { post.z == pre.z })
// TODO             &&& (b ==> pre.y <= pre.z) && {
// TODO                 &&& c
// TODO                 &&& pre.x == post.x
// TODO                 &&& pre.y == post.y
// TODO             }
// TODO         }
// TODO
// TODO         proof fn correct_init(post: X::State, x: int, y: int, z: int) {
// TODO             requires(X::State::initialize(post, x, y, z));
// TODO             ensures(rel_init(post, x, y, z));
// TODO         }
// TODO
// TODO         proof fn rev_init(post: X::State, x: int, y: int, z: int) {
// TODO             requires(rel_init(post, x, y, z));
// TODO             ensures(X::State::initialize(post, x, y, z));
// TODO         }
// TODO
// TODO         proof fn correct_tr1(pre: X::State, post: X::State, b: bool, c: bool) {
// TODO             requires(X::State::tr1(pre, post, b, c));
// TODO             ensures(rel_tr1(pre, post, b, c));
// TODO         }
// TODO
// TODO         proof fn rev_tr1(pre: X::State, post: X::State, b: bool, c: bool) {
// TODO             requires(rel_tr1(pre, post, b, c));
// TODO             ensures(X::State::tr1(pre, post, b, c));
// TODO         }
// TODO
// TODO         proof fn correct_tr1_strong(pre: X::State, post: X::State, b: bool, c: bool) {
// TODO             requires(X::State::tr1_strong(pre, post, b, c));
// TODO             ensures(rel_tr1_strong(pre, post, b, c));
// TODO         }
// TODO
// TODO         proof fn rev_tr1_strong(pre: X::State, post: X::State, b: bool, c: bool) {
// TODO             requires(rel_tr1_strong(pre, post, b, c));
// TODO             ensures(X::State::tr1_strong(pre, post, b, c));
// TODO         }
// TODO
// TODO         proof fn correct_tr2(pre: X::State, post: X::State, b: bool, c: bool) {
// TODO             requires(X::State::tr2(pre, post, b, c));
// TODO             ensures(rel_tr2(pre, post, b, c));
// TODO         }
// TODO
// TODO         proof fn rev_tr2(pre: X::State, post: X::State, b: bool, c: bool) {
// TODO             requires(rel_tr2(pre, post, b, c));
// TODO             ensures(X::State::tr2(pre, post, b, c));
// TODO         }
// TODO
// TODO         proof fn correct_tr2_strong(pre: X::State, post: X::State, b: bool, c: bool) {
// TODO             requires(X::State::tr2_strong(pre, post, b, c));
// TODO             ensures(rel_tr2_strong(pre, post, b, c));
// TODO         }
// TODO
// TODO         proof fn rev_tr2_strong(pre: X::State, post: X::State, b: bool, c: bool) {
// TODO             requires(rel_tr2_strong(pre, post, b, c));
// TODO             ensures(X::State::tr2_strong(pre, post, b, c));
// TODO         }
// TODO
// TODO         proof fn correct_tr3(pre: X::State, post: X::State, b: bool, c: bool) {
// TODO             requires(X::State::tr3(pre, post, b, c));
// TODO             ensures(rel_tr3(pre, post, b, c));
// TODO         }
// TODO
// TODO         proof fn rev_tr3(pre: X::State, post: X::State, b: bool, c: bool) {
// TODO             requires(rel_tr3(pre, post, b, c));
// TODO             ensures(X::State::tr3(pre, post, b, c));
// TODO         }
// TODO
// TODO         proof fn correct_tr3_strong(pre: X::State, post: X::State, b: bool, c: bool) {
// TODO             requires(X::State::tr3_strong(pre, post, b, c));
// TODO             ensures(rel_tr3_strong(pre, post, b, c));
// TODO         }
// TODO
// TODO         proof fn rev_tr3_strong(pre: X::State, post: X::State, b: bool, c: bool) {
// TODO             requires(rel_tr3_strong(pre, post, b, c));
// TODO             ensures(X::State::tr3_strong(pre, post, b, c));
// TODO         }
// TODO
// TODO         } // verus!
// TODO     } => Ok(())
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] relation_codegen_match IMPORTS.to_string() + verus_code_str! {
// TODO         state_machine!{ X {
// TODO             fields {
// TODO                 pub x: int,
// TODO                 pub y: int,
// TODO                 pub z: int,
// TODO             }
// TODO
// TODO             init!{
// TODO                 initialize(x: int, y: int, z: int, foo: Foo) {
// TODO                     init x = x;
// TODO                     init y = y;
// TODO                     require(y <= z);
// TODO                     match foo {
// TODO                         Foo::Bar(a) => { init z = z; }
// TODO                         Foo::Qax(b) => { init z = z + 1; }
// TODO                         Foo::Duck(d) => { init z = z + 2; }
// TODO                     }
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr1(foo: Foo, c: bool) {
// TODO                     match foo {
// TODO                         Foo::Bar(a) => { update z = pre.z + 1; }
// TODO                         Foo::Qax(b) if b == 20 => { assert(pre.y <= pre.z); }
// TODO                         Foo::Duck(d) => { assert(foo.is_Duck()); }
// TODO                         _ => { }
// TODO                     }
// TODO                     require(c);
// TODO                 }
// TODO             }
// TODO
// TODO             #[invariant]
// TODO             pub fn the_inv(self) -> bool { self.y <= self.z }
// TODO
// TODO             #[inductive(initialize)]
// TODO             fn init_inductive(post: Self, x: int, y: int, z: int, foo: Foo) { }
// TODO
// TODO             #[inductive(tr1)]
// TODO             fn tr1_inductive(pre: Self, post: Self, foo: Foo, c: bool) { }
// TODO         }}
// TODO
// TODO         verus! {
// TODO
// TODO         spec fn rel_init(post: X::State, x: int, y: int, z: int, foo: Foo) -> bool {
// TODO             &&& post.x == x
// TODO             &&& post.y == y
// TODO             &&& y <= z
// TODO             &&& match foo {
// TODO                 Foo::Bar(a) => { post.z == z }
// TODO                 Foo::Qax(b) => { post.z == z + 1 }
// TODO                 Foo::Duck(d) => { post.z == z + 2 }
// TODO             }
// TODO         }
// TODO
// TODO         spec fn rel_tr1(pre: X::State, post: X::State, foo: Foo, c: bool) -> bool {
// TODO             &&& (match foo {
// TODO                 Foo::Bar(a) => { post.z == pre.z + 1 }
// TODO                 Foo::Qax(b) if b == 20 => { pre.y <= pre.z ==> post.z == pre.z }
// TODO                 Foo::Duck(d) => { post.z == pre.z }
// TODO                 _ => { post.z == pre.z }
// TODO             })
// TODO             &&& ((match foo {
// TODO                 Foo::Bar(a) => { true }
// TODO                 Foo::Qax(b) if b == 20 => { pre.y <= pre.z }
// TODO                 Foo::Duck(d) => { true }
// TODO                 _ => { true }
// TODO             }) ==> {
// TODO                 &&& c
// TODO                 &&& pre.x == post.x
// TODO                 &&& pre.y == post.y
// TODO             })
// TODO         }
// TODO
// TODO         spec fn rel_tr1_strong(pre: X::State, post: X::State, foo: Foo, c: bool) -> bool {
// TODO             &&& (match foo {
// TODO                 Foo::Bar(a) => { post.z == pre.z + 1 }
// TODO                 Foo::Qax(b) if b == 20 => { post.z == pre.z && pre.y <= pre.z }
// TODO                 Foo::Duck(d) => { post.z == pre.z }
// TODO                 _ => { post.z == pre.z }
// TODO             })
// TODO             &&& (c && pre.x == post.x && pre.y == post.y)
// TODO         }
// TODO
// TODO         proof fn correct_init(post: X::State, x: int, y: int, z: int, foo: Foo) {
// TODO             requires(X::State::initialize(post, x, y, z, foo));
// TODO             ensures(rel_init(post, x, y, z, foo));
// TODO         }
// TODO
// TODO         proof fn rev_init(post: X::State, x: int, y: int, z: int, foo: Foo) {
// TODO             requires(rel_init(post, x, y, z, foo));
// TODO             ensures(X::State::initialize(post, x, y, z, foo));
// TODO         }
// TODO
// TODO         proof fn correct_tr1(pre: X::State, post: X::State, foo: Foo, c: bool) {
// TODO             requires(X::State::tr1(pre, post, foo, c));
// TODO             ensures(rel_tr1(pre, post, foo, c));
// TODO         }
// TODO
// TODO         proof fn rev_tr1(pre: X::State, post: X::State, foo: Foo, c: bool) {
// TODO             requires(rel_tr1(pre, post, foo, c));
// TODO             ensures(X::State::tr1(pre, post, foo, c));
// TODO         }
// TODO
// TODO         proof fn correct_tr1_strong(pre: X::State, post: X::State, foo: Foo, c: bool) {
// TODO             requires(X::State::tr1_strong(pre, post, foo, c));
// TODO             ensures(rel_tr1_strong(pre, post, foo, c));
// TODO         }
// TODO
// TODO         proof fn rev_tr1_strong(pre: X::State, post: X::State, foo: Foo, c: bool) {
// TODO             requires(rel_tr1_strong(pre, post, foo, c));
// TODO             ensures(X::State::tr1_strong(pre, post, foo, c));
// TODO         }
// TODO
// TODO         } // verus!
// TODO
// TODO     } => Ok(())
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] relation_codegen_special IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ Y {
// TODO             fields {
// TODO                 #[sharding(option)]
// TODO                 pub opt: Option<int>,
// TODO
// TODO                 #[sharding(map)]
// TODO                 pub map: Map<int, int>,
// TODO
// TODO                 #[sharding(multiset)]
// TODO                 pub mset: Multiset<int>,
// TODO
// TODO                 #[sharding(storage_option)]
// TODO                 pub storage_opt: Option<int>,
// TODO
// TODO                 #[sharding(storage_map)]
// TODO                 pub storage_map: Map<int, int>,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr1() {
// TODO                     remove opt -= Some(5);
// TODO                     add opt += Some(8);
// TODO
// TODO                     remove map -= [0 => 1];
// TODO                     have map >= [2 => 3];
// TODO                     add map += [4 => 5] by { assume(false); };
// TODO
// TODO                     remove mset -= {10};
// TODO                     have mset >= {11};
// TODO                     add mset += {12};
// TODO
// TODO                     withdraw storage_opt -= Some(13) by { assume(false); };
// TODO                     deposit storage_opt += Some(14);
// TODO
// TODO                     withdraw storage_map -= [15 => 16] by { assume(false); };
// TODO                     deposit storage_map += [17 => 18] by { assume(false); };
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr2() {
// TODO                     have opt >= Some(7);
// TODO                     add map += [4 => 5] by { assume(false); };
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr3() {
// TODO                     remove opt -= Some(7);
// TODO                     withdraw storage_opt -= Some(12) by { assume(false); };
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr4() {
// TODO                     add opt += Some(7) by { assume(false); };
// TODO                     deposit storage_opt += Some(12) by { assume(false); };
// TODO                 }
// TODO             }
// TODO         }}
// TODO
// TODO         verus! {
// TODO
// TODO         spec fn rel_tr1(pre: Y::State, post: Y::State) -> bool {
// TODO             &&& pre.opt === Option::Some(5)
// TODO             &&& pre.map.contains_pair(0, 1)
// TODO             &&& pre.map.remove(0).contains_pair(2, 3)
// TODO             &&& !pre.map.remove(0).dom().contains(4)
// TODO               ==> pre.mset.count(10) >= 1
// TODO               && pre.mset.remove(10).count(11) >= 1
// TODO               && (pre.storage_opt === Option::Some(13)
// TODO                 ==> (pre.storage_map.contains_pair(15, 16)
// TODO                   ==> (!pre.storage_map.remove(15).dom().contains(17)
// TODO                     ==> post.storage_map === pre.storage_map.remove(15).insert(17, 18)
// TODO                      && post.opt === Option::Some(8)
// TODO                      && post.map === pre.map.remove(0).insert(4, 5)
// TODO                      && post.mset === pre.mset.remove(10).insert(12)
// TODO                      && post.storage_opt === Option::Some(14)
// TODO                   )))
// TODO         }
// TODO
// TODO         spec fn rel_tr1_strong(pre: Y::State, post: Y::State) -> bool {
// TODO             &&& pre.opt === Option::Some(5)
// TODO             &&& post.opt === Option::Some(8)
// TODO
// TODO             &&& pre.map.contains_pair(0, 1)
// TODO             &&& pre.map.remove(0).contains_pair(2, 3)
// TODO             &&& !pre.map.remove(0).dom().contains(4)
// TODO             &&& post.map === pre.map.remove(0).insert(4, 5)
// TODO
// TODO             &&& pre.mset.count(10) >= 1
// TODO             &&& pre.mset.remove(10).count(11) >= 1
// TODO             &&& post.mset === pre.mset.remove(10).insert(12)
// TODO
// TODO             &&& pre.storage_opt === Option::Some(13)
// TODO             &&& post.storage_opt === Option::Some(14)
// TODO
// TODO             &&& pre.storage_map.contains_pair(15, 16)
// TODO             &&& !pre.storage_map.remove(15).dom().contains(17)
// TODO             &&& post.storage_map === pre.storage_map.remove(15).insert(17, 18)
// TODO         }
// TODO
// TODO         spec fn rel_tr2(pre: Y::State, post: Y::State) -> bool {
// TODO             &&& pre.opt === Option::Some(7)
// TODO             &&& !pre.map.dom().contains(4) ==> {
// TODO                 &&& post.map === pre.map.insert(4, 5)
// TODO                 &&& post.opt === pre.opt
// TODO                 &&& post.storage_opt === pre.storage_opt
// TODO                 &&& post.storage_map === pre.storage_map
// TODO                 &&& post.mset === pre.mset
// TODO             }
// TODO         }
// TODO
// TODO         spec fn rel_tr2_strong(pre: Y::State, post: Y::State) -> bool {
// TODO             &&& pre.opt === Option::Some(7)
// TODO             &&& !pre.map.dom().contains(4)
// TODO             &&& post.map === pre.map.insert(4, 5)
// TODO             &&& post.opt === pre.opt
// TODO             &&& post.storage_opt === pre.storage_opt
// TODO             &&& post.storage_map === pre.storage_map
// TODO             &&& post.mset === pre.mset
// TODO         }
// TODO
// TODO         spec fn rel_tr3(pre: Y::State, post: Y::State) -> bool {
// TODO             &&& pre.opt === Option::Some(7)
// TODO             &&& pre.storage_opt === Option::Some(12)
// TODO               ==> post.storage_opt === Option::None
// TODO                 && post.map === pre.map
// TODO                 && post.storage_map === pre.storage_map
// TODO                 && post.mset === pre.mset
// TODO                 && post.opt === Option::None
// TODO         }
// TODO
// TODO         spec fn rel_tr3_strong(pre: Y::State, post: Y::State) -> bool {
// TODO             &&& pre.opt === Option::Some(7)
// TODO             &&& post.opt === Option::None
// TODO             &&& pre.storage_opt === Option::Some(12)
// TODO             &&& post.storage_opt === Option::None
// TODO             &&& post.map === pre.map
// TODO             &&& post.storage_map === pre.storage_map
// TODO             &&& post.mset === pre.mset
// TODO         }
// TODO
// TODO         spec fn rel_tr4(pre: Y::State, post: Y::State) -> bool {
// TODO             pre.opt === Option::None ==> (
// TODO               (pre.storage_opt === Option::None ==> {
// TODO                 &&& post.storage_opt === Option::Some(12)
// TODO                 &&& post.map === pre.map
// TODO                 &&& post.storage_map === pre.storage_map
// TODO                 &&& post.mset === pre.mset
// TODO                 &&& post.opt === Option::Some(7)
// TODO               })
// TODO             )
// TODO         }
// TODO
// TODO         spec fn rel_tr4_strong(pre: Y::State, post: Y::State) -> bool {
// TODO             &&& pre.opt === Option::None
// TODO             &&& post.opt === Option::Some(7)
// TODO             &&& pre.storage_opt === Option::None
// TODO             &&& post.storage_opt === Option::Some(12)
// TODO             &&& post.map === pre.map
// TODO             &&& post.storage_map === pre.storage_map
// TODO             &&& post.mset === pre.mset
// TODO         }
// TODO
// TODO         proof fn correct_tr1(pre: Y::State, post: Y::State) {
// TODO             requires(Y::State::tr1(pre, post));
// TODO             ensures(rel_tr1(pre, post));
// TODO         }
// TODO
// TODO         proof fn rev_tr1(pre: Y::State, post: Y::State) {
// TODO             requires(rel_tr1(pre, post));
// TODO             ensures(Y::State::tr1(pre, post));
// TODO         }
// TODO
// TODO         proof fn correct_tr1_strong(pre: Y::State, post: Y::State) {
// TODO             requires(Y::State::tr1_strong(pre, post));
// TODO             ensures(rel_tr1_strong(pre, post));
// TODO         }
// TODO
// TODO         proof fn rev_tr1_strong(pre: Y::State, post: Y::State) {
// TODO             requires(rel_tr1_strong(pre, post));
// TODO             ensures(Y::State::tr1_strong(pre, post));
// TODO         }
// TODO
// TODO         proof fn correct_tr2(pre: Y::State, post: Y::State) {
// TODO             requires(Y::State::tr2(pre, post));
// TODO             ensures(rel_tr2(pre, post));
// TODO         }
// TODO
// TODO         proof fn rev_tr2(pre: Y::State, post: Y::State) {
// TODO             requires(rel_tr2(pre, post));
// TODO             ensures(Y::State::tr2(pre, post));
// TODO         }
// TODO
// TODO         proof fn correct_tr2_strong(pre: Y::State, post: Y::State) {
// TODO             requires(Y::State::tr2_strong(pre, post));
// TODO             ensures(rel_tr2_strong(pre, post));
// TODO         }
// TODO
// TODO         proof fn rev_tr2_strong(pre: Y::State, post: Y::State) {
// TODO             requires(rel_tr2_strong(pre, post));
// TODO             ensures(Y::State::tr2_strong(pre, post));
// TODO         }
// TODO
// TODO         proof fn correct_tr3(pre: Y::State, post: Y::State) {
// TODO             requires(Y::State::tr3(pre, post));
// TODO             ensures(rel_tr3(pre, post));
// TODO         }
// TODO
// TODO         proof fn rev_tr3(pre: Y::State, post: Y::State) {
// TODO             requires(rel_tr3(pre, post));
// TODO             ensures(Y::State::tr3(pre, post));
// TODO         }
// TODO
// TODO         proof fn correct_tr3_strong(pre: Y::State, post: Y::State) {
// TODO             requires(Y::State::tr3_strong(pre, post));
// TODO             ensures(rel_tr3_strong(pre, post));
// TODO         }
// TODO
// TODO         proof fn rev_tr3_strong(pre: Y::State, post: Y::State) {
// TODO             requires(rel_tr3_strong(pre, post));
// TODO             ensures(Y::State::tr3_strong(pre, post));
// TODO         }
// TODO
// TODO         proof fn correct_tr4(pre: Y::State, post: Y::State) {
// TODO             requires(Y::State::tr4(pre, post));
// TODO             ensures(rel_tr4(pre, post));
// TODO         }
// TODO
// TODO         proof fn rev_tr4(pre: Y::State, post: Y::State) {
// TODO             requires(rel_tr4(pre, post));
// TODO             ensures(Y::State::tr4(pre, post));
// TODO         }
// TODO
// TODO         proof fn correct_tr4_strong(pre: Y::State, post: Y::State) {
// TODO             requires(Y::State::tr4_strong(pre, post));
// TODO             ensures(rel_tr4_strong(pre, post));
// TODO         }
// TODO
// TODO         proof fn rev_tr4_strong(pre: Y::State, post: Y::State) {
// TODO             requires(rel_tr4_strong(pre, post));
// TODO             ensures(Y::State::tr4_strong(pre, post));
// TODO         }
// TODO
// TODO         } // verus!
// TODO
// TODO     } => Ok(())
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] relation_codegen_special_general IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ Y {
// TODO             fields {
// TODO                 #[sharding(option)]
// TODO                 pub opt: Option<int>,
// TODO
// TODO                 #[sharding(map)]
// TODO                 pub map: Map<int, int>,
// TODO
// TODO                 #[sharding(multiset)]
// TODO                 pub mset: Multiset<int>,
// TODO
// TODO                 #[sharding(storage_option)]
// TODO                 pub storage_opt: Option<int>,
// TODO
// TODO                 #[sharding(storage_map)]
// TODO                 pub storage_map: Map<int, int>,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr1() {
// TODO                     remove opt -= ( Option::Some(5) );
// TODO                     add opt += ( Option::Some(8) );
// TODO
// TODO                     remove map -= ( Map::<int, int>::empty().insert(0, 1) );
// TODO                     have map >= ( Map::<int, int>::empty().insert(2, 3) );
// TODO                     add map += ( Map::<int, int>::empty().insert(4, 5) ) by { assume(false); };
// TODO
// TODO                     remove mset -= ( Multiset::<int>::singleton(10) );
// TODO                     have mset >= ( Multiset::<int>::singleton(11) );
// TODO                     add mset += ( Multiset::<int>::singleton(12) );
// TODO
// TODO                     withdraw storage_opt -= ( Option::Some(13) ) by { assume(false); };
// TODO                     deposit storage_opt += ( Option::Some(14) );
// TODO
// TODO                     withdraw storage_map -= ( Map::<int, int>::empty().insert(15, 16) ) by { assume(false); };
// TODO                     deposit storage_map += ( Map::<int, int>::empty().insert(17, 18) ) by { assume(false); };
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr2() {
// TODO                     have opt >= (Option::Some(7));
// TODO                     add map += (Map::<int, int>::empty().insert(4, 5)) by { assume(false); };
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr3() {
// TODO                     remove opt -= (Option::Some(7));
// TODO                     withdraw storage_opt -= (Option::Some(12)) by { assume(false); };
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr4() {
// TODO                     add opt += (Option::Some(7)) by { assume(false); };
// TODO                     deposit storage_opt += (Option::Some(12)) by { assume(false); };
// TODO                 }
// TODO             }
// TODO         }}
// TODO
// TODO         verus! {
// TODO
// TODO         spec fn rel_tr1(pre: Y::State, post: Y::State) -> bool {
// TODO             &&& pre.opt === Option::Some(5)
// TODO
// TODO             &&& map![0 => 1].le(pre.map)
// TODO             &&& map![2 => 3].le(pre.map.remove_keys(map![0 => 1int].dom()))
// TODO             &&& pre.map.remove_keys(map![0 => 1int].dom()).dom().disjoint(map![4 => 5int].dom())
// TODO
// TODO             ==> {
// TODO
// TODO             &&& Multiset::singleton(10).le(pre.mset)
// TODO             &&& Multiset::singleton(11).le(pre.mset.sub(Multiset::singleton(10)))
// TODO
// TODO             &&& (pre.storage_opt === Option::Some(13)
// TODO
// TODO             ==>
// TODO
// TODO             (map![15 => 16].le(pre.storage_map)
// TODO
// TODO             ==>
// TODO
// TODO             (pre.storage_map.remove_keys(map![15 => 16int].dom()).dom().disjoint(map![17 => 18int].dom())
// TODO
// TODO             ==> {
// TODO
// TODO             &&& post.opt === Option::Some(8)
// TODO             &&& post.map === pre.map.remove_keys(map![0 => 1int].dom()).union_prefer_right(map![4 => 5])
// TODO             &&& post.mset ===
// TODO                 pre.mset.sub(Multiset::singleton(10)).add(Multiset::singleton(12))
// TODO             &&& post.storage_opt === Option::Some(14)
// TODO             &&& post.storage_map ===
// TODO                 pre.storage_map.remove_keys(map![15 => 16int].dom()).union_prefer_right(map![17 => 18])
// TODO             })))}
// TODO         }
// TODO
// TODO         spec fn rel_tr1_strong(pre: Y::State, post: Y::State) -> bool {
// TODO             &&& pre.opt === Option::Some(5)
// TODO             &&& post.opt === Option::Some(8)
// TODO
// TODO             &&& map![0 => 1].le(pre.map)
// TODO             &&& map![2 => 3].le(pre.map.remove_keys(map![0 => 1int].dom()))
// TODO             &&& pre.map.remove_keys(map![0 => 1int].dom()).dom().disjoint(map![4 => 5int].dom())
// TODO             &&& post.map === pre.map.remove_keys(map![0 => 1int].dom()).union_prefer_right(map![4 => 5])
// TODO
// TODO             &&& Multiset::singleton(10).le(pre.mset)
// TODO             &&& Multiset::singleton(11).le(pre.mset.sub(Multiset::singleton(10)))
// TODO             &&& post.mset ===
// TODO                 pre.mset.sub(Multiset::singleton(10)).add(Multiset::singleton(12))
// TODO
// TODO             &&& pre.storage_opt === Option::Some(13)
// TODO             &&& post.storage_opt === Option::Some(14)
// TODO
// TODO             &&& map![15 => 16].le(pre.storage_map)
// TODO             &&& pre.storage_map.remove_keys(map![15 => 16int].dom()).dom().disjoint(map![17 => 18int].dom())
// TODO             &&& post.storage_map ===
// TODO                 pre.storage_map.remove_keys(map![15 => 16int].dom()).union_prefer_right(map![17 => 18])
// TODO         }
// TODO
// TODO         spec fn rel_tr2(pre: Y::State, post: Y::State) -> bool {
// TODO             &&& pre.opt === Option::Some(7)
// TODO             &&& !pre.map.dom().contains(4) ==> {
// TODO                 &&& post.map === pre.map.union_prefer_right(map![4 => 5])
// TODO                 &&& post.opt === pre.opt
// TODO                 &&& post.storage_opt === pre.storage_opt
// TODO                 &&& post.storage_map === pre.storage_map
// TODO                 &&& post.mset === pre.mset
// TODO             }
// TODO         }
// TODO
// TODO         spec fn rel_tr2_strong(pre: Y::State, post: Y::State) -> bool {
// TODO             &&& pre.opt === Option::Some(7)
// TODO             &&& !pre.map.dom().contains(4)
// TODO             &&& post.map === pre.map.union_prefer_right(map![4 => 5])
// TODO             &&& post.opt === pre.opt
// TODO             &&& post.storage_opt === pre.storage_opt
// TODO             &&& post.storage_map === pre.storage_map
// TODO             &&& post.mset === pre.mset
// TODO         }
// TODO
// TODO         spec fn rel_tr3(pre: Y::State, post: Y::State) -> bool {
// TODO             &&& pre.opt === Option::Some(7)
// TODO             &&& pre.storage_opt === Option::Some(12) ==> {
// TODO                 &&& post.storage_opt === Option::None
// TODO                 &&& post.map === pre.map
// TODO                 &&& post.storage_map === pre.storage_map
// TODO                 &&& post.mset === pre.mset
// TODO                 &&& post.opt === Option::None
// TODO             }
// TODO         }
// TODO
// TODO         spec fn rel_tr3_strong(pre: Y::State, post: Y::State) -> bool {
// TODO             &&& pre.opt === Option::Some(7)
// TODO             &&& post.opt === Option::None
// TODO             &&& pre.storage_opt === Option::Some(12)
// TODO             &&& post.storage_opt === Option::None
// TODO             &&& post.map === pre.map
// TODO             &&& post.storage_map === pre.storage_map
// TODO             &&& post.mset === pre.mset
// TODO         }
// TODO
// TODO         spec fn rel_tr4(pre: Y::State, post: Y::State) -> bool {
// TODO             pre.opt === Option::None ==> (
// TODO               (pre.storage_opt === Option::None ==> {
// TODO                 &&& post.storage_opt === Option::Some(12)
// TODO                 &&& post.map === pre.map
// TODO                 &&& post.storage_map === pre.storage_map
// TODO                 &&& post.mset === pre.mset
// TODO                 &&& post.opt === Option::Some(7)
// TODO               })
// TODO             )
// TODO         }
// TODO
// TODO         spec fn rel_tr4_strong(pre: Y::State, post: Y::State) -> bool {
// TODO             &&& pre.opt === Option::None
// TODO             &&& post.opt === Option::Some(7)
// TODO             &&& pre.storage_opt === Option::None
// TODO             &&& post.storage_opt === Option::Some(12)
// TODO             &&& post.map === pre.map
// TODO             &&& post.storage_map === pre.storage_map
// TODO             &&& post.mset === pre.mset
// TODO         }
// TODO
// TODO         proof fn correct_tr1(pre: Y::State, post: Y::State) {
// TODO             requires(Y::State::tr1(pre, post));
// TODO             ensures(rel_tr1(pre, post));
// TODO         }
// TODO
// TODO         proof fn rev_tr1(pre: Y::State, post: Y::State) {
// TODO             requires(rel_tr1(pre, post));
// TODO             ensures(Y::State::tr1(pre, post));
// TODO         }
// TODO
// TODO         proof fn correct_tr1_strong(pre: Y::State, post: Y::State) {
// TODO             requires(Y::State::tr1_strong(pre, post));
// TODO             ensures(rel_tr1_strong(pre, post));
// TODO         }
// TODO
// TODO         proof fn rev_tr1_strong(pre: Y::State, post: Y::State) {
// TODO             requires(rel_tr1_strong(pre, post));
// TODO             ensures(Y::State::tr1_strong(pre, post));
// TODO         }
// TODO
// TODO         proof fn correct_tr2(pre: Y::State, post: Y::State) {
// TODO             requires(Y::State::tr2(pre, post));
// TODO             ensures(rel_tr2(pre, post));
// TODO         }
// TODO
// TODO         proof fn rev_tr2(pre: Y::State, post: Y::State) {
// TODO             requires(rel_tr2(pre, post));
// TODO             ensures(Y::State::tr2(pre, post));
// TODO         }
// TODO
// TODO         proof fn correct_tr2_strong(pre: Y::State, post: Y::State) {
// TODO             requires(Y::State::tr2_strong(pre, post));
// TODO             ensures(rel_tr2_strong(pre, post));
// TODO         }
// TODO
// TODO         proof fn rev_tr2_strong(pre: Y::State, post: Y::State) {
// TODO             requires(rel_tr2_strong(pre, post));
// TODO             ensures(Y::State::tr2_strong(pre, post));
// TODO         }
// TODO
// TODO         proof fn correct_tr3(pre: Y::State, post: Y::State) {
// TODO             requires(Y::State::tr3(pre, post));
// TODO             ensures(rel_tr3(pre, post));
// TODO         }
// TODO
// TODO         proof fn rev_tr3(pre: Y::State, post: Y::State) {
// TODO             requires(rel_tr3(pre, post));
// TODO             ensures(Y::State::tr3(pre, post));
// TODO         }
// TODO
// TODO         proof fn correct_tr3_strong(pre: Y::State, post: Y::State) {
// TODO             requires(Y::State::tr3_strong(pre, post));
// TODO             ensures(rel_tr3_strong(pre, post));
// TODO         }
// TODO
// TODO         proof fn rev_tr3_strong(pre: Y::State, post: Y::State) {
// TODO             requires(rel_tr3_strong(pre, post));
// TODO             ensures(Y::State::tr3_strong(pre, post));
// TODO         }
// TODO
// TODO         proof fn correct_tr4(pre: Y::State, post: Y::State) {
// TODO             requires(Y::State::tr4(pre, post));
// TODO             ensures(rel_tr4(pre, post));
// TODO         }
// TODO
// TODO         proof fn rev_tr4(pre: Y::State, post: Y::State) {
// TODO             requires(rel_tr4(pre, post));
// TODO             ensures(Y::State::tr4(pre, post));
// TODO         }
// TODO
// TODO         proof fn correct_tr4_strong(pre: Y::State, post: Y::State) {
// TODO             requires(Y::State::tr4_strong(pre, post));
// TODO             ensures(rel_tr4_strong(pre, post));
// TODO         }
// TODO
// TODO         proof fn rev_tr4_strong(pre: Y::State, post: Y::State) {
// TODO             requires(rel_tr4_strong(pre, post));
// TODO             ensures(Y::State::tr4_strong(pre, post));
// TODO         }
// TODO
// TODO         } // verus!
// TODO     } => Ok(())
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] relation_codegen_opt_general IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ Y {
// TODO             fields {
// TODO                 #[sharding(option)]
// TODO                 pub opt: Option<int>,
// TODO
// TODO                 #[sharding(storage_option)]
// TODO                 pub storage_opt: Option<int>,
// TODO             }
// TODO
// TODO             property!{
// TODO                 ro() {
// TODO                     guard storage_opt >= (Option::<int>::None);
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr1() {
// TODO                     have opt >= (Option::<int>::None);
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr2() {
// TODO                     add opt += (Option::<int>::None);
// TODO                     deposit storage_opt += (Option::<int>::None);
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr3() {
// TODO                     remove opt -= (Option::<int>::None);
// TODO                     withdraw storage_opt -= (Option::<int>::None);
// TODO                 }
// TODO             }
// TODO         }}
// TODO
// TODO         verus!{
// TODO
// TODO         spec fn rel_tr1(pre: Y::State, post: Y::State) -> bool {
// TODO             equal(pre.opt, post.opt) && equal(pre.storage_opt, post.storage_opt)
// TODO         }
// TODO
// TODO         spec fn rel_tr1_strong(pre: Y::State, post: Y::State) -> bool {
// TODO             equal(pre.opt, post.opt) && equal(pre.storage_opt, post.storage_opt)
// TODO         }
// TODO
// TODO         spec fn rel_tr2(pre: Y::State, post: Y::State) -> bool {
// TODO             equal(pre.opt, post.opt) && equal(pre.storage_opt, post.storage_opt)
// TODO         }
// TODO
// TODO         spec fn rel_tr2_strong(pre: Y::State, post: Y::State) -> bool {
// TODO             equal(pre.opt, post.opt) && equal(pre.storage_opt, post.storage_opt)
// TODO         }
// TODO
// TODO         spec fn rel_tr3(pre: Y::State, post: Y::State) -> bool {
// TODO             equal(pre.opt, post.opt) && equal(pre.storage_opt, post.storage_opt)
// TODO         }
// TODO
// TODO         spec fn rel_tr3_strong(pre: Y::State, post: Y::State) -> bool {
// TODO             equal(pre.opt, post.opt) && equal(pre.storage_opt, post.storage_opt)
// TODO         }
// TODO
// TODO         proof fn correct_tr1(pre: Y::State, post: Y::State) {
// TODO             requires(Y::State::tr1(pre, post));
// TODO             ensures(rel_tr1(pre, post));
// TODO         }
// TODO
// TODO         proof fn rev_tr1(pre: Y::State, post: Y::State) {
// TODO             requires(rel_tr1(pre, post));
// TODO             ensures(Y::State::tr1(pre, post));
// TODO         }
// TODO
// TODO         proof fn correct_tr1_strong(pre: Y::State, post: Y::State) {
// TODO             requires(Y::State::tr1_strong(pre, post));
// TODO             ensures(rel_tr1_strong(pre, post));
// TODO         }
// TODO
// TODO         proof fn rev_tr1_strong(pre: Y::State, post: Y::State) {
// TODO             requires(rel_tr1_strong(pre, post));
// TODO             ensures(Y::State::tr1_strong(pre, post));
// TODO         }
// TODO
// TODO         proof fn correct_tr2(pre: Y::State, post: Y::State) {
// TODO             requires(Y::State::tr2(pre, post));
// TODO             ensures(rel_tr2(pre, post));
// TODO         }
// TODO
// TODO         proof fn rev_tr2(pre: Y::State, post: Y::State) {
// TODO             requires(rel_tr2(pre, post));
// TODO             ensures(Y::State::tr2(pre, post));
// TODO         }
// TODO
// TODO         proof fn correct_tr2_strong(pre: Y::State, post: Y::State) {
// TODO             requires(Y::State::tr2_strong(pre, post));
// TODO             ensures(rel_tr2_strong(pre, post));
// TODO         }
// TODO
// TODO         proof fn rev_tr2_strong(pre: Y::State, post: Y::State) {
// TODO             requires(rel_tr2_strong(pre, post));
// TODO             ensures(Y::State::tr2_strong(pre, post));
// TODO         }
// TODO
// TODO         proof fn correct_tr3(pre: Y::State, post: Y::State) {
// TODO             requires(Y::State::tr3(pre, post));
// TODO             ensures(rel_tr3(pre, post));
// TODO         }
// TODO
// TODO         proof fn rev_tr3(pre: Y::State, post: Y::State) {
// TODO             requires(rel_tr3(pre, post));
// TODO             ensures(Y::State::tr3(pre, post));
// TODO         }
// TODO
// TODO         proof fn correct_tr3_strong(pre: Y::State, post: Y::State) {
// TODO             requires(Y::State::tr3_strong(pre, post));
// TODO             ensures(rel_tr3_strong(pre, post));
// TODO         }
// TODO
// TODO         proof fn rev_tr3_strong(pre: Y::State, post: Y::State) {
// TODO             requires(rel_tr3_strong(pre, post));
// TODO             ensures(Y::State::tr3_strong(pre, post));
// TODO         }
// TODO
// TODO         }
// TODO     } => Ok(())
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] nondet_tokenizing IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ Z {
// TODO             fields {
// TODO                 #[sharding(variable)]
// TODO                 pub v1: int,
// TODO
// TODO                 #[sharding(variable)]
// TODO                 pub v2: int,
// TODO
// TODO                 #[sharding(not_tokenized)]
// TODO                 pub nt: int,
// TODO
// TODO                 #[sharding(constant)]
// TODO                 pub c: int,
// TODO             }
// TODO
// TODO             init!{
// TODO                 initialize() {
// TODO                     init v1 = 0;
// TODO                     init v2 = 1;
// TODO                     init nt = 2;
// TODO                     init c = 3;
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr1() {
// TODO                     update nt = pre.nt + 1; // this is ok because the exchange fn ignores this line
// TODO                     update v1 = pre.v1 + 2;
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr2() {
// TODO                     // v1 should be passed in as tokens, v2 read nondeterministically
// TODO                     birds_eye let x = pre.nt + pre.c + pre.v1 - pre.v2;
// TODO                     update v1 = x;
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr3() {
// TODO                     // v1, v2 both passed in as tokens
// TODO                     birds_eye let x = pre.nt + pre.c + pre.v1 - pre.v2;
// TODO                     update v1 = x + 4 * pre.v2;
// TODO                 }
// TODO             }
// TODO         }}
// TODO
// TODO         verus!{
// TODO         proof fn go() {
// TODO             let tracked (Tracked(instance), Tracked(mut v1), Tracked(v2)) = Z::Instance::initialize();
// TODO             assert(equal(v1.view().instance, instance));
// TODO             assert(equal(v2.view().instance, instance));
// TODO             assert(equal(v1.view().value, spec_literal_int("0")));
// TODO             assert(equal(v2.view().value, spec_literal_int("1")));
// TODO             assert(equal(instance.c(), spec_literal_int("3")));
// TODO
// TODO             instance.tr1(&mut v1);
// TODO             assert(equal(v1.view().instance, instance));
// TODO             assert(equal(v1.view().value, spec_literal_int("2")));
// TODO
// TODO             let old_v1_value = v1.view().value;
// TODO             let tracked (Ghost(birds_eye_v2), Ghost(birds_eye_nt)) = instance.tr2(&mut v1);
// TODO             assert(equal(v1.view().instance, instance));
// TODO             assert(equal(v1.view().value,
// TODO                 birds_eye_nt + instance.c() + old_v1_value - birds_eye_v2));
// TODO
// TODO             let old_v1_value = v1.view().value;
// TODO             let birds_eye_nt = instance.tr3(&mut v1, &v2);
// TODO             assert(equal(v1.view().instance, instance));
// TODO             assert(equal(v1.view().value, birds_eye_nt + instance.c() + old_v1_value + spec_literal_int("3") * v2.view().value));
// TODO         }
// TODO         }
// TODO     } => Ok(())
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] pre_in_init IMPORTS.to_string() + verus_code_str! {
// TODO         state_machine!{ X {
// TODO             fields {
// TODO                 pub t: int,
// TODO             }
// TODO
// TODO             init!{
// TODO                 init() {
// TODO                     update t = pre.t;
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "no previous state to refer to")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] self_in_transition IMPORTS.to_string() + verus_code_str! {
// TODO         state_machine!{ X {
// TODO             fields {
// TODO                 pub t: int,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     update t = self.t;
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "`self` is meaningless")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] post_in_transition IMPORTS.to_string() + verus_code_str! {
// TODO         state_machine!{ X {
// TODO             fields {
// TODO                 pub t: int,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     update t = post.t;
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "cannot refer directly to `post`")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] test_let_pattern IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields { #[sharding(variable)] pub t: (int, int) }
// TODO
// TODO             init!{
// TODO                 initialize() {
// TODO                     init t = (2, 2);
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr() {
// TODO                     let (a, b) = pre.t;
// TODO                     update t = (a + 1, b + 1);
// TODO                 }
// TODO             }
// TODO
// TODO             #[invariant]
// TODO             pub fn inv(&self) -> bool {
// TODO                 self.t.0 == self.t.1
// TODO             }
// TODO
// TODO             #[inductive(initialize)]
// TODO             fn initialize_inductive(post: Self) { }
// TODO
// TODO             #[inductive(tr)]
// TODO             fn tr_inductive(pre: Self, post: Self) { }
// TODO         }}
// TODO     } => Ok(())
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] super_error IMPORTS.to_string() + verus_code_str! {
// TODO         struct Bar { }
// TODO
// TODO         state_machine!{ X {
// TODO             fields { pub t: int }
// TODO
// TODO             transition!{
// TODO                 // this is disallowed because the macro would try to copy the path into
// TODO                 // an inner module
// TODO                 tr(foo: super::Bar) {
// TODO                     update t = 5;
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "`super::` path not allowed here")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] if_let_fail IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(storage_option)] pub so: Option<int>
// TODO             }
// TODO
// TODO             property!{
// TODO                 tr() {
// TODO                     if let x = 5 {
// TODO                         assert(x == 5);
// TODO                     }
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "do not support if-let conditionals")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] if_let_fail_with_else IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(storage_option)] pub so: Option<int>
// TODO             }
// TODO
// TODO             property!{
// TODO                 tr() {
// TODO                     if let x = 5 {
// TODO                         assert(x == 5);
// TODO                     } else {
// TODO                         assert(true);
// TODO                     }
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "do not support if-let conditionals")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] if_let_fail_with_chain IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ X {
// TODO             fields {
// TODO                 #[sharding(storage_option)] pub so: Option<int>
// TODO             }
// TODO
// TODO             property!{
// TODO                 tr() {
// TODO                     if true && let x = 5 {
// TODO                         assert(x == 5);
// TODO                     } else {
// TODO                         assert(true);
// TODO                     }
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "do not support if-let conditionals")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] use_self_type IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ Y {
// TODO             fields {
// TODO                 #[sharding(variable)]
// TODO                 pub x: int,
// TODO
// TODO                 #[sharding(variable)]
// TODO                 pub recursing: Option<Box<Self>>,
// TODO             }
// TODO
// TODO             init!{
// TODO                 ini(t: Self) {
// TODO                     let r: Self = t;
// TODO                     init x = r.x;
// TODO                     init recursing = t.recursing;
// TODO                 }
// TODO             }
// TODO
// TODO             pub open spec fn add1(x: int) -> int {
// TODO                 x + 1
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr(a: int) {
// TODO                     update x = Self::add1(a);
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr2(y: Option<Box<Self>>) {
// TODO                     let t: Option<Box<Self>> = y;
// TODO                     update recursing = t;
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr3() {
// TODO                     update recursing = Option::<Box<Self>>::None;
// TODO                 }
// TODO             }
// TODO
// TODO         }}
// TODO
// TODO         verus!{
// TODO         pub proof fn foo() {
// TODO             let tracked (Tracked(inst), Tracked(mut x_tok), Tracked(mut r_tok)) = Y::Instance::ini(
// TODO                 Y::State { x: spec_literal_int("5"), recursing: Option::None }
// TODO             );
// TODO             inst.tr(spec_literal_int("19"), &mut x_tok);
// TODO             assert(x_tok.view().value == spec_literal_int("20"));
// TODO
// TODO             inst.tr2(Option::<Box<Y::State>>::None, &mut r_tok);
// TODO             assert(equal(Option::<Box<Y::State>>::None, r_tok.view().value));
// TODO
// TODO             inst.tr3(&mut r_tok);
// TODO             assert(equal(Option::<Box<Y::State>>::None, r_tok.view().value));
// TODO         }
// TODO         }
// TODO     } => Ok(())
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] bind_codegen IMPORTS.to_string() + verus_code_str! {
// TODO
// TODO         tokenized_state_machine!{ Y {
// TODO             fields {
// TODO                 #[sharding(option)]
// TODO                 pub opt: Option<int>,
// TODO
// TODO                 #[sharding(map)]
// TODO                 pub map: Map<int, u64>,
// TODO
// TODO                 #[sharding(storage_map)]
// TODO                 pub storage_map: Map<int, u64>,
// TODO             }
// TODO
// TODO             init!{
// TODO                 initialize() {
// TODO                     init opt = Option::Some(2);
// TODO                     init map = Map::<int,u64>::empty().insert(1, 5);
// TODO                     init storage_map = Map::<int,u64>::empty().insert(1, 6);
// TODO                 }
// TODO             }
// TODO
// TODO             #[invariant]
// TODO             pub fn maps_eq(&self) -> bool {
// TODO                 equal(self.map.dom(), self.storage_map.dom())
// TODO             }
// TODO
// TODO             #[invariant]
// TODO             pub fn maps_6(&self) -> bool {
// TODO                 forall |k| imply(self.storage_map.dom().contains(k),
// TODO                     self.storage_map.index(k) == 6)
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr1() {
// TODO                     remove opt -= Some(let x);
// TODO                     require(x == 2);
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr2(key: int) {
// TODO                     remove map -= [key => let x];
// TODO                     require(x == 5);
// TODO
// TODO                     withdraw storage_map -= [key => let y];
// TODO                     assert(y == 6);
// TODO                 }
// TODO             }
// TODO
// TODO             readonly!{
// TODO                 tr3(key: int) {
// TODO                     have map >= [key => let x];
// TODO                     require(x == 5);
// TODO
// TODO                     guard storage_map >= [key => 6];
// TODO                 }
// TODO             }
// TODO
// TODO             property!{
// TODO                 tr4(key: int) {
// TODO                     have map >= [key => let x];
// TODO                     require(x == 5);
// TODO
// TODO                     guard storage_map >= [key => 6];
// TODO                 }
// TODO             }
// TODO
// TODO             #[inductive(initialize)]
// TODO             fn initialize_inductive(post: Self) { }
// TODO
// TODO             #[inductive(tr1)]
// TODO             fn tr1_inductive(pre: Self, post: Self) { }
// TODO
// TODO             #[inductive(tr2)]
// TODO             fn tr2_inductive(pre: Self, post: Self, key: int) { }
// TODO         }}
// TODO
// TODO         verus! {
// TODO         spec fn rel_tr1(pre: Y::State, post: Y::State) -> bool {
// TODO             &&& pre.opt === Option::Some(2)
// TODO             &&& post.opt === Option::None
// TODO             &&& post.map === pre.map
// TODO             &&& post.storage_map === pre.storage_map
// TODO         }
// TODO
// TODO         spec fn rel_tr1_strong(pre: Y::State, post: Y::State) -> bool {
// TODO             &&& pre.opt === Option::Some(2)
// TODO             &&& post.opt === Option::None
// TODO             &&& post.map === pre.map
// TODO             &&& post.storage_map === pre.storage_map
// TODO         }
// TODO
// TODO         spec fn rel_tr2(pre: Y::State, post: Y::State, key: int) -> bool {
// TODO             &&& pre.map.dom().contains(key)
// TODO             &&& pre.map.index(key) == 5
// TODO
// TODO             &&& (
// TODO               (pre.storage_map.dom().contains(key) && pre.storage_map.index(key) == 6)
// TODO               ==> {
// TODO                 &&& post.map === pre.map.remove(key)
// TODO                 &&& post.storage_map === pre.storage_map.remove(key)
// TODO                 &&& post.opt === pre.opt
// TODO               }
// TODO            )
// TODO         }
// TODO
// TODO         spec fn rel_tr2_strong(pre: Y::State, post: Y::State, key: int) -> bool {
// TODO             &&& pre.map.dom().contains(key)
// TODO             &&& pre.map.index(key) == 5
// TODO             &&& (
// TODO               (pre.storage_map.dom().contains(key) && pre.storage_map.index(key) == 6)
// TODO               && {
// TODO                 &&& post.map === pre.map.remove(key)
// TODO                 &&& post.storage_map === pre.storage_map.remove(key)
// TODO                 &&& post.opt === pre.opt
// TODO               }
// TODO            )
// TODO         }
// TODO
// TODO         spec fn rel_tr3(pre: Y::State, post: Y::State, key: int) -> bool {
// TODO             &&& pre.map.dom().contains(key)
// TODO             &&& pre.map.index(key) == 5
// TODO
// TODO             &&& (
// TODO               (pre.storage_map.dom().contains(key) && pre.storage_map.index(key) == 6)
// TODO               ==> {
// TODO                 &&& post.map === pre.map
// TODO                 &&& post.storage_map === pre.storage_map
// TODO                 &&& post.opt === pre.opt
// TODO               }
// TODO            )
// TODO         }
// TODO
// TODO         spec fn rel_tr3_strong(pre: Y::State, post: Y::State, key: int) -> bool {
// TODO             &&& pre.map.dom().contains(key)
// TODO             &&& pre.map.index(key) == 5
// TODO
// TODO             &&& (
// TODO               (pre.storage_map.dom().contains(key) && pre.storage_map.index(key) == 6)
// TODO               && {
// TODO                 &&& post.map === pre.map
// TODO                 &&& post.storage_map === pre.storage_map
// TODO                 &&& post.opt === pre.opt
// TODO               }
// TODO            )
// TODO         }
// TODO
// TODO         proof fn correct_tr1(pre: Y::State, post: Y::State) {
// TODO             requires(Y::State::tr1(pre, post));
// TODO             ensures(rel_tr1(pre, post));
// TODO         }
// TODO
// TODO         proof fn rev_tr1(pre: Y::State, post: Y::State) {
// TODO             requires(rel_tr1(pre, post));
// TODO             ensures(Y::State::tr1(pre, post));
// TODO         }
// TODO
// TODO         proof fn correct_tr1_strong(pre: Y::State, post: Y::State) {
// TODO             requires(Y::State::tr1_strong(pre, post));
// TODO             ensures(rel_tr1_strong(pre, post));
// TODO         }
// TODO
// TODO         proof fn rev_tr1_strong(pre: Y::State, post: Y::State) {
// TODO             requires(rel_tr1_strong(pre, post));
// TODO             ensures(Y::State::tr1_strong(pre, post));
// TODO         }
// TODO
// TODO         proof fn correct_tr2(pre: Y::State, post: Y::State, key: int) {
// TODO             requires(Y::State::tr2(pre, post, key));
// TODO             ensures(rel_tr2(pre, post, key));
// TODO         }
// TODO
// TODO         proof fn rev_tr2(pre: Y::State, post: Y::State, key: int) {
// TODO             requires(rel_tr2(pre, post, key));
// TODO             ensures(Y::State::tr2(pre, post, key));
// TODO         }
// TODO
// TODO         proof fn correct_tr2_strong(pre: Y::State, post: Y::State, key: int) {
// TODO             requires(Y::State::tr2_strong(pre, post, key));
// TODO             ensures(rel_tr2_strong(pre, post, key));
// TODO         }
// TODO
// TODO         proof fn rev_tr2_strong(pre: Y::State, post: Y::State, key: int) {
// TODO             requires(rel_tr2_strong(pre, post, key));
// TODO             ensures(Y::State::tr2_strong(pre, post, key));
// TODO         }
// TODO
// TODO         proof fn correct_tr3(pre: Y::State, post: Y::State, key: int) {
// TODO             requires(Y::State::tr3(pre, post, key));
// TODO             ensures(rel_tr3(pre, post, key));
// TODO         }
// TODO
// TODO         proof fn rev_tr3(pre: Y::State, post: Y::State, key: int) {
// TODO             requires(rel_tr3(pre, post, key));
// TODO             ensures(Y::State::tr3(pre, post, key));
// TODO         }
// TODO
// TODO         proof fn correct_tr3_strong(pre: Y::State, post: Y::State, key: int) {
// TODO             requires(Y::State::tr3_strong(pre, post, key));
// TODO             ensures(rel_tr3_strong(pre, post, key));
// TODO         }
// TODO
// TODO         proof fn rev_tr3_strong(pre: Y::State, post: Y::State, key: int) {
// TODO             requires(rel_tr3_strong(pre, post, key));
// TODO             ensures(Y::State::tr3_strong(pre, post, key));
// TODO         }
// TODO
// TODO         proof fn do_tokens() {
// TODO             let tracked mut m: Map<int, u64> = Map::tracked_empty();
// TODO             m.tracked_insert(spec_literal_int("1"), 6u64);
// TODO             let tracked (Tracked(inst), Tracked(opt_token), Tracked(mut map_tokens)) = Y::Instance::initialize(m);
// TODO
// TODO             match opt_token {
// TODO                 Option::None => { assert(false); }
// TODO                 Option::Some(opt_token) => {
// TODO                     inst.tr1(opt_token);
// TODO
// TODO                     assert(map_tokens.dom().contains(spec_literal_int("1")));
// TODO                     let tracked map_token = map_tokens.tracked_remove(spec_literal_int("1"));
// TODO
// TODO                     let tracked the_guard = inst.tr4(spec_literal_int("1"), &map_token);
// TODO                     assert(*the_guard == 6);
// TODO
// TODO                     let tracked t = inst.tr2(spec_literal_int("1"), map_token);
// TODO                     assert(t == 6);
// TODO                 }
// TODO             };
// TODO         }
// TODO
// TODO         } // verus!
// TODO
// TODO     } => Ok(())
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] bind_fail_add IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ Y {
// TODO             fields {
// TODO                 #[sharding(option)]
// TODO                 pub opt: Option<int>,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr1() {
// TODO                     add opt += Some(let x);
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "pattern-binding cannot be used in an 'add' statement")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] bind_fail_deposit IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ Y {
// TODO             fields {
// TODO                 #[sharding(storage_option)]
// TODO                 pub opt: Option<int>,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr1() {
// TODO                     deposit opt += Some(let x);
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "pattern-binding cannot be used in a 'deposit' statement")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] bind_fail_guard IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ Y {
// TODO             fields {
// TODO                 #[sharding(storage_option)]
// TODO                 pub opt: Option<int>,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr1() {
// TODO                     guard opt >= Some(let x);
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "pattern-binding cannot be used in a 'guard' statement")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] assert_let_fail_1_bind IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ Y {
// TODO             fields {
// TODO                 #[sharding(variable)]
// TODO                 pub opt: Option<int>,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr1() {
// TODO                     assert let Option::Some(x) = pre.opt;
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "unable to prove safety condition that the pattern matches")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] assert_let_fail_0_bind IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ Y {
// TODO             fields {
// TODO                 #[sharding(variable)]
// TODO                 pub opt: Option<int>,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr1() {
// TODO                     assert let Option::Some(_) = pre.opt;
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "unable to prove safety condition that the pattern matches")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] assert_require_let_codegen IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ Y {
// TODO             fields {
// TODO                 #[sharding(variable)]
// TODO                 pub opt1: Option<int>,
// TODO
// TODO                 #[sharding(variable)]
// TODO                 pub opt2: Option<int>,
// TODO
// TODO                 #[sharding(variable)]
// TODO                 pub opt3: Option<int>,
// TODO
// TODO                 #[sharding(variable)]
// TODO                 pub opt4: Option<int>,
// TODO             }
// TODO
// TODO             init!{
// TODO                 initialize() {
// TODO                     init opt1 = Option::Some(0);
// TODO                     init opt2 = Option::Some(5);
// TODO                     init opt3 = Option::None;
// TODO                     init opt4 = Option::Some(5);
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr1() {
// TODO                     require let (Option::Some(x), Option::Some(y)) = (pre.opt1, pre.opt2);
// TODO                     assert let (Option::None, Option::Some(z)) = (pre.opt3, pre.opt4);
// TODO
// TODO                     assert(y == z);
// TODO
// TODO                     update opt1 = Option::None;
// TODO                     update opt3 = Option::Some(x + y + z);
// TODO                 }
// TODO             }
// TODO
// TODO             #[invariant]
// TODO             pub fn the_inv(&self) -> bool {
// TODO                 imply(self.opt1.is_Some(), (
// TODO                     self.opt2.is_Some()
// TODO                         && self.opt4.is_Some()
// TODO                         && equal(self.opt2, self.opt4)
// TODO                         && self.opt3.is_None()
// TODO                 ))
// TODO             }
// TODO
// TODO             #[inductive(initialize)]
// TODO             fn initialize_inductive(post: Self) { }
// TODO
// TODO             #[inductive(tr1)]
// TODO             fn tr1_inductive(pre: Self, post: Self) { }
// TODO         }}
// TODO
// TODO         verus! {
// TODO
// TODO         spec fn rel_tr1(pre: Y::State, post: Y::State) -> bool {
// TODO             match (pre.opt1, pre.opt2) {
// TODO                 (Option::Some(x), Option::Some(y)) => {
// TODO                     match (pre.opt3, pre.opt4) {
// TODO                         (Option::None, Option::Some(z)) => {
// TODO                             y == z ==> {
// TODO                                 &&& post.opt1 === Option::None
// TODO                                 &&& post.opt2 === pre.opt2
// TODO                                 &&& post.opt3 === Option::Some(x + y + z)
// TODO                                 &&& post.opt4 === pre.opt4
// TODO                             }
// TODO                         }
// TODO                         _ => {
// TODO                             true
// TODO                         }
// TODO                     }
// TODO                 }
// TODO                 _ => {
// TODO                     false
// TODO                 }
// TODO             }
// TODO         }
// TODO
// TODO         spec fn rel_tr1_strong(pre: Y::State, post: Y::State) -> bool {
// TODO             match (pre.opt1, pre.opt2) {
// TODO                 (Option::Some(x), Option::Some(y)) => {
// TODO                     match (pre.opt3, pre.opt4) {
// TODO                         (Option::None, Option::Some(z)) => {
// TODO                             y == z &&
// TODO                             equal(post.opt1, Option::None) &&
// TODO                             equal(post.opt2, pre.opt2) &&
// TODO                             equal(post.opt3, Option::Some(x + y + z)) &&
// TODO                             equal(post.opt4, pre.opt4)
// TODO                         }
// TODO                         _ => {
// TODO                             false
// TODO                         }
// TODO                     }
// TODO                 }
// TODO                 _ => {
// TODO                     false
// TODO                 }
// TODO             }
// TODO         }
// TODO
// TODO         proof fn correct_tr1(pre: Y::State, post: Y::State) {
// TODO             requires(Y::State::tr1(pre, post));
// TODO             ensures(rel_tr1(pre, post));
// TODO         }
// TODO
// TODO         proof fn rev_tr1(pre: Y::State, post: Y::State) {
// TODO             requires(rel_tr1(pre, post));
// TODO             ensures(Y::State::tr1(pre, post));
// TODO         }
// TODO
// TODO         proof fn correct_tr1_strong(pre: Y::State, post: Y::State) {
// TODO             requires(Y::State::tr1_strong(pre, post));
// TODO             ensures(rel_tr1_strong(pre, post));
// TODO         }
// TODO
// TODO         proof fn rev_tr1_strong(pre: Y::State, post: Y::State) {
// TODO             requires(rel_tr1_strong(pre, post));
// TODO             ensures(Y::State::tr1_strong(pre, post));
// TODO         }
// TODO
// TODO         proof fn test_transition(
// TODO             tracked inst: Y::Instance,
// TODO             tracked t1: Y::opt1,
// TODO             tracked t2: Y::opt2,
// TODO             tracked t3: Y::opt3,
// TODO             tracked t4: Y::opt4
// TODO         ) {
// TODO             requires([
// TODO                 equal(inst, t1@.instance),
// TODO                 equal(inst, t2@.instance),
// TODO                 equal(inst, t3@.instance),
// TODO                 equal(inst, t4@.instance),
// TODO                 equal(t1@.value, Option::Some(0)),
// TODO                 equal(t2@.value, Option::Some(5)),
// TODO             ]);
// TODO
// TODO             let old_t1 = t1;
// TODO             let old_t3 = t3;
// TODO
// TODO             let tracked mut t1 = t1;
// TODO             let tracked mut t3 = t3;
// TODO
// TODO             inst.tr1(&mut t1, &t2, &mut t3, &t4);
// TODO
// TODO             assert(equal(old_t3@.value, Option::None));
// TODO             assert(equal(t4@.value, Option::Some(5)));
// TODO             assert(equal(t1@.value, Option::None));
// TODO             assert(equal(t3@.value, Option::Some(10)));
// TODO         }
// TODO
// TODO         proof fn test_start() {
// TODO             let tracked (Tracked(inst), Tracked(t1), Tracked(t2), Tracked(t3), Tracked(t4)) = Y::Instance::initialize();
// TODO             test_transition(inst, t1, t2, t3, t4);
// TODO         }
// TODO
// TODO         } // verus!
// TODO
// TODO     } => Ok(())
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] count_codegen IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ Y {
// TODO             fields {
// TODO                 #[sharding(count)]
// TODO                 pub c: nat,
// TODO             }
// TODO
// TODO             init!{
// TODO                 initialize() {
// TODO                     init c = 9;
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr_add() {
// TODO                     add c += (spec_literal_nat("2"));
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr_have() {
// TODO                     have c >= (spec_literal_nat("2"));
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr_remove() {
// TODO                     remove c -= (spec_literal_nat("2"));
// TODO                 }
// TODO             }
// TODO         }}
// TODO
// TODO         verus!{
// TODO
// TODO         spec fn rel_tr1(pre: Y::State, post: Y::State) -> bool {
// TODO             post.c == pre.c + spec_literal_nat("2")
// TODO         }
// TODO
// TODO         spec fn rel_tr1_strong(pre: Y::State, post: Y::State) -> bool {
// TODO             post.c == pre.c + spec_literal_nat("2")
// TODO         }
// TODO
// TODO         spec fn rel_tr2(pre: Y::State, post: Y::State) -> bool {
// TODO             pre.c >= spec_literal_nat("2") && post.c == pre.c
// TODO         }
// TODO
// TODO         spec fn rel_tr2_strong(pre: Y::State, post: Y::State) -> bool {
// TODO             pre.c >= spec_literal_nat("2") && post.c == pre.c
// TODO         }
// TODO
// TODO         spec fn rel_tr3(pre: Y::State, post: Y::State) -> bool {
// TODO             pre.c >= spec_literal_nat("2") && post.c == pre.c - spec_literal_nat("2")
// TODO         }
// TODO
// TODO         spec fn rel_tr3_strong(pre: Y::State, post: Y::State) -> bool {
// TODO             pre.c >= spec_literal_nat("2") && post.c == pre.c - spec_literal_nat("2")
// TODO         }
// TODO
// TODO         proof fn correct_tr(pre: Y::State, post: Y::State) {
// TODO             ensures([
// TODO                 rel_tr1(pre, post) == Y::State::tr_add(pre, post),
// TODO                 rel_tr1_strong(pre, post) == Y::State::tr_add_strong(pre, post),
// TODO                 rel_tr2(pre, post) == Y::State::tr_have(pre, post),
// TODO                 rel_tr2_strong(pre, post) == Y::State::tr_have_strong(pre, post),
// TODO                 rel_tr3(pre, post) == Y::State::tr_remove(pre, post),
// TODO                 rel_tr3_strong(pre, post) == Y::State::tr_remove_strong(pre, post),
// TODO             ]);
// TODO         }
// TODO
// TODO         proof fn test_inst() {
// TODO             let tracked (Tracked(inst), Tracked(t1)) = Y::Instance::initialize();
// TODO             assert(t1.view().count == spec_literal_nat("9"));
// TODO
// TODO             let tracked (Tracked(t2), Tracked(t3)) = t1.split(spec_literal_nat("2"));
// TODO
// TODO             assert(t2.view().count == spec_literal_nat("2"));
// TODO             assert(t3.view().count == spec_literal_nat("7"));
// TODO
// TODO             inst.tr_have(&t2);
// TODO             inst.tr_remove(t2);
// TODO
// TODO             let tracked t4 = inst.tr_add();
// TODO             assert(t4.view().count == spec_literal_nat("2"));
// TODO
// TODO             let tracked q = t4.join(t3);
// TODO             assert(q.view().count == spec_literal_nat("9"));
// TODO         }
// TODO
// TODO         proof fn test_join_fail() {
// TODO             let tracked (Tracked(inst1), Tracked(t1)) = Y::Instance::initialize();
// TODO             let tracked (Tracked(inst2), Tracked(t2)) = Y::Instance::initialize();
// TODO             let tracked t = t1.join(t2); // FAILS
// TODO         }
// TODO
// TODO         proof fn test_split_fail() {
// TODO             let tracked (Tracked(inst), Tracked(t1)) = Y::Instance::initialize();
// TODO
// TODO             let tracked (Tracked(t2), Tracked(t3)) = t1.split(spec_literal_nat("10")); // FAILS
// TODO         }
// TODO
// TODO         }
// TODO     } => Err(e) => assert_fails(e, 2)
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] persistent_option_remove_fail IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ Y {
// TODO             fields {
// TODO                 #[sharding(persistent_option)]
// TODO                 pub c: Option<int>,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr_remove() {
// TODO                     remove c -= Some(1);
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "a persistent field's value can only grow, never remove or modify its data")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] persistent_map_remove_fail IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ Y {
// TODO             fields {
// TODO                 #[sharding(persistent_map)]
// TODO                 pub c: Map<int, int>,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr_remove() {
// TODO                     remove c -= [1 => 2];
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "a persistent field's value can only grow, never remove or modify its data")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] persistent_bool_remove_fail IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ Y {
// TODO             fields {
// TODO                 #[sharding(persistent_bool)]
// TODO                 pub c: bool,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr_remove() {
// TODO                     remove c -= true;
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "a persistent field's value can only grow, never remove or modify its data")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] use_plus_for_persistent_fail IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ Y {
// TODO             fields {
// TODO                 #[sharding(persistent_bool)]
// TODO                 pub c: bool,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr_add() {
// TODO                     add c += true;
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "for the persistent strategy `persistent_bool`, use `(union)=` instead of `+=`")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] use_union_for_nonpersistent_fail IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ Y {
// TODO             fields {
// TODO                 #[sharding(bool)]
// TODO                 pub c: bool,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr_add() {
// TODO                     add c (union)= true;
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "use `+=` instead of `(union)=`")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] persistent_count_remove_fail IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ Y {
// TODO             fields {
// TODO                 #[sharding(persistent_count)]
// TODO                 pub c: nat,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr_remove() {
// TODO                     remove c -= (1);
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "a persistent field's value can only grow, never remove or modify its data")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] persistent_set_remove_fail IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ Y {
// TODO             fields {
// TODO                 #[sharding(persistent_set)]
// TODO                 pub c: Set<int>,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr_remove() {
// TODO                     remove c -= set { 1 };
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "a persistent field's value can only grow, never remove or modify its data")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] persistent_option_codegen IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ Y {
// TODO             fields {
// TODO                 #[sharding(persistent_option)]
// TODO                 pub c: Option<int>,
// TODO
// TODO                 #[sharding(persistent_option)]
// TODO                 pub d: Option<int>,
// TODO             }
// TODO
// TODO             init!{
// TODO                 initialize() {
// TODO                     init c = Option::None;
// TODO                     init d = Option::Some(7);
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr1() {
// TODO                     have d >= Some(7);
// TODO                     add c (union)= Some(3);
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr2() {
// TODO                     add c (union)= ( Option::Some(3) );
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr3() {
// TODO                     have c >= (
// TODO                         Option::Some(3)
// TODO                     );
// TODO                 }
// TODO             }
// TODO
// TODO             #[invariant]
// TODO             pub fn the_inv(&self) -> bool {
// TODO                 (match self.c {
// TODO                     Option::Some(x) => x == 3,
// TODO                     _ => true,
// TODO                 })
// TODO                 &&
// TODO                 (match self.d {
// TODO                     Option::Some(x) => x == 7,
// TODO                     _ => true,
// TODO                 })
// TODO             }
// TODO
// TODO             #[inductive(initialize)]
// TODO             fn initialize_inductive(post: Self) { }
// TODO
// TODO             #[inductive(tr1)]
// TODO             fn tr1_inductive(pre: Self, post: Self) { }
// TODO
// TODO             #[inductive(tr2)]
// TODO             fn tr2_inductive(pre: Self, post: Self) { }
// TODO
// TODO             #[inductive(tr3)]
// TODO             fn tr3_inductive(pre: Self, post: Self) { }
// TODO         }}
// TODO
// TODO         verus! {
// TODO         spec fn rel_tr1(pre: Y::State, post: Y::State) -> bool {
// TODO             &&& pre.d === Option::Some(7)
// TODO             &&& (
// TODO                 (match pre.c {
// TODO                     Option::Some(x) => x == 3,
// TODO                     Option::None => true,
// TODO                 })
// TODO                 ==> {
// TODO                     &&& pre.d === post.d
// TODO                     &&& post.c === Option::Some(3)
// TODO                 }
// TODO             )
// TODO         }
// TODO
// TODO         spec fn rel_tr1_strong(pre: Y::State, post: Y::State) -> bool {
// TODO             &&& pre.d === Option::Some(7)
// TODO             &&& (
// TODO                 (match pre.c {
// TODO                     Option::Some(x) => x == 3,
// TODO                     Option::None => true,
// TODO                 })
// TODO                 && {
// TODO                     &&& pre.d === post.d
// TODO                     &&& post.c === Option::Some(3)
// TODO                 }
// TODO             )
// TODO         }
// TODO
// TODO         spec fn rel_tr2(pre: Y::State, post: Y::State) -> bool {
// TODO             (match pre.c {
// TODO                 Option::Some(x) => x == 3,
// TODO                 Option::None => true,
// TODO             })
// TODO             ==> {
// TODO                 &&& pre.d === post.d
// TODO                 &&& post.c === Option::Some(3)
// TODO             }
// TODO         }
// TODO
// TODO         spec fn rel_tr2_strong(pre: Y::State, post: Y::State) -> bool {
// TODO             &&& (match pre.c {
// TODO                 Option::Some(x) => x == 3,
// TODO                 Option::None => true,
// TODO             })
// TODO             &&& {
// TODO                 &&& pre.d === post.d
// TODO                 &&& post.c === Option::Some(3)
// TODO             }
// TODO         }
// TODO
// TODO         spec fn rel_tr3(pre: Y::State, post: Y::State) -> bool {
// TODO             &&& pre.c === Option::Some(3)
// TODO             &&& post.c === pre.c
// TODO             &&& post.d === pre.d
// TODO         }
// TODO
// TODO         spec fn rel_tr3_strong(pre: Y::State, post: Y::State) -> bool {
// TODO             rel_tr3(pre, post)
// TODO         }
// TODO
// TODO         proof fn correct_tr(pre: Y::State, post: Y::State) {
// TODO             ensures([
// TODO                 rel_tr1(pre, post) == Y::State::tr1(pre, post),
// TODO                 rel_tr1_strong(pre, post) == Y::State::tr1_strong(pre, post),
// TODO                 rel_tr2(pre, post) == Y::State::tr2(pre, post),
// TODO                 rel_tr2_strong(pre, post) == Y::State::tr2_strong(pre, post),
// TODO                 rel_tr3(pre, post) == Y::State::tr3(pre, post),
// TODO                 rel_tr3_strong(pre, post) == Y::State::tr3_strong(pre, post),
// TODO             ]);
// TODO         }
// TODO
// TODO         proof fn test_inst() {
// TODO             let tracked (Tracked(inst), Tracked(_c), Tracked(d_opt)) = Y::Instance::initialize();
// TODO
// TODO             let tracked d = match d_opt {
// TODO                 Option::Some(d) => d,
// TODO                 Option::None => proof_from_false(),
// TODO             };
// TODO
// TODO             let tracked cloned = d.clone();
// TODO             assert(equal(cloned.view().instance, inst));
// TODO             assert(d.view().value == spec_literal_int("7"));
// TODO
// TODO             let tracked c = inst.tr1(&d);
// TODO             assert(c.view().value == spec_literal_int("3"));
// TODO             assert(equal(c.view().instance, inst));
// TODO
// TODO             let tracked c2_opt = inst.tr2();
// TODO             let tracked c2 = match c2_opt {
// TODO                 Option::Some(c2) => c2,
// TODO                 Option::None => proof_from_false(),
// TODO             };
// TODO             assert(c2.view().value == spec_literal_int("3"));
// TODO             assert(equal(c2.view().instance, inst));
// TODO
// TODO             let tracked c_opt = Option::Some(c);
// TODO             inst.tr3(&c_opt);
// TODO         }
// TODO
// TODO         } // verus!
// TODO
// TODO     } => Ok(())
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] persistent_map_codegen IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ Y {
// TODO             fields {
// TODO                 #[sharding(persistent_map)]
// TODO                 pub c: Map<int, int>,
// TODO             }
// TODO
// TODO             init!{
// TODO                 initialize() {
// TODO                     init c = Map::empty().insert(1, 2);
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr1() {
// TODO                     have c >= [1 => 2];
// TODO                     add c (union)= [3 => 4];
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr2() {
// TODO                     add c (union)= (
// TODO                         Map::empty().insert(5, 9).insert(12, 15)
// TODO                     );
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr3() {
// TODO                     have c >= (
// TODO                         Map::empty().insert(5, 9).insert(12, 15)
// TODO                     );
// TODO                 }
// TODO             }
// TODO
// TODO             #[invariant]
// TODO             pub fn the_inv(&self) -> bool {
// TODO                 imply(self.c.dom().contains(5), self.c.index(5) == 9)
// TODO                 &&
// TODO                 imply(self.c.dom().contains(12), self.c.index(12) == 15)
// TODO                 &&
// TODO                 imply(self.c.dom().contains(3), self.c.index(3) == 4)
// TODO             }
// TODO
// TODO             #[inductive(initialize)]
// TODO             fn initialize_inductive(post: Self) { }
// TODO
// TODO             #[inductive(tr1)]
// TODO             fn tr1_inductive(pre: Self, post: Self) { }
// TODO
// TODO             #[inductive(tr2)]
// TODO             fn tr2_inductive(pre: Self, post: Self) { }
// TODO
// TODO             #[inductive(tr3)]
// TODO             fn tr3_inductive(pre: Self, post: Self) { }
// TODO         }}
// TODO
// TODO         verus!{
// TODO         spec fn rel_tr1(pre: Y::State, post: Y::State) -> bool {
// TODO             &&& pre.c.dom().contains(1)
// TODO             &&& pre.c.index(1) == 2
// TODO             &&& (
// TODO               (pre.c.dom().contains(3) ==> pre.c.index(3) == 4)
// TODO               ==> (
// TODO                 post.c === pre.c.insert(3, 4)
// TODO               )
// TODO             )
// TODO         }
// TODO
// TODO         spec fn rel_tr1_strong(pre: Y::State, post: Y::State) -> bool {
// TODO             &&& pre.c.dom().contains(1)
// TODO             &&& pre.c.index(1) == 2
// TODO             &&& (
// TODO               (pre.c.dom().contains(3) ==> pre.c.index(3) == 4)
// TODO               && (
// TODO                 post.c === pre.c.insert(3, 4)
// TODO               )
// TODO             )
// TODO         }
// TODO
// TODO         spec fn rel_tr2(pre: Y::State, post: Y::State) -> bool {
// TODO             ((pre.c.dom().contains(5) ==> pre.c.index(5) == 9)
// TODO             && (pre.c.dom().contains(12) ==> pre.c.index(12) == 15))
// TODO             ==> post.c ===
// TODO                   pre.c.insert(5, 9).insert(12, 15)
// TODO         }
// TODO
// TODO         spec fn rel_tr2_strong(pre: Y::State, post: Y::State) -> bool {
// TODO             ((pre.c.dom().contains(5) ==> pre.c.index(5) == 9)
// TODO             && (pre.c.dom().contains(12) ==> pre.c.index(12) == 15))
// TODO             && post.c ===
// TODO                   pre.c.insert(5, 9).insert(12, 15)
// TODO         }
// TODO
// TODO         spec fn rel_tr3(pre: Y::State, post: Y::State) -> bool {
// TODO             &&& ((pre.c.dom().contains(5) && pre.c.index(5) == 9)
// TODO             &&& (pre.c.dom().contains(12) && pre.c.index(12) == 15))
// TODO             &&& pre.c === post.c
// TODO         }
// TODO
// TODO         spec fn rel_tr3_strong(pre: Y::State, post: Y::State) -> bool {
// TODO             &&& ((pre.c.dom().contains(5) && pre.c.index(5) == 9)
// TODO             &&& (pre.c.dom().contains(12) && pre.c.index(12) == 15))
// TODO             &&& pre.c === post.c
// TODO         }
// TODO
// TODO         proof fn correct_tr(pre: Y::State, post: Y::State)
// TODO             ensures
// TODO                 rel_tr1(pre, post) == Y::State::tr1(pre, post),
// TODO                 rel_tr1_strong(pre, post) == Y::State::tr1_strong(pre, post),
// TODO                 rel_tr2(pre, post) == Y::State::tr2(pre, post),
// TODO                 rel_tr2_strong(pre, post) == Y::State::tr2_strong(pre, post),
// TODO                 rel_tr3(pre, post) == Y::State::tr3(pre, post),
// TODO                 rel_tr3_strong(pre, post) == Y::State::tr3_strong(pre, post),
// TODO         {
// TODO             assert_maps_equal!(
// TODO                 pre.c.insert(5, 9).insert(12, 15),
// TODO                 pre.c.union_prefer_right(
// TODO                     Map::empty().insert(5, 9).insert(12, 15)
// TODO                 )
// TODO             );
// TODO
// TODO             if rel_tr3(pre, post) {
// TODO                 assert(
// TODO                   Map::empty().insert(5, 9).insert(12, 15).le(pre.c)
// TODO                 );
// TODO                 assert(Y::State::tr3(pre, post));
// TODO             }
// TODO             if Y::State::tr3(pre, post) {
// TODO                 assert(
// TODO                   Map::<int, int>::empty().insert(5, 9).insert(12, 15).dom().contains(5)
// TODO                 );
// TODO                 assert(
// TODO                   Map::<int, int>::empty().insert(5, 9).insert(12, 15).dom().contains(12)
// TODO                 );
// TODO                 assert(pre.c.dom().contains(5));
// TODO                 assert(pre.c.dom().contains(12));
// TODO                 assert(rel_tr3(pre, post));
// TODO             }
// TODO         }
// TODO
// TODO         proof fn test_inst() {
// TODO             let tracked (Tracked(inst), Tracked(mut init_m)) = Y::Instance::initialize();
// TODO             assert(init_m.dom().contains(spec_literal_int("1")));
// TODO             let tracked m_1 = init_m.tracked_remove(spec_literal_int("1"));
// TODO             assert(m_1.view().value == spec_literal_int("2"));
// TODO
// TODO             let tracked cloned = m_1.clone();
// TODO             assert(equal(cloned.view().instance, inst));
// TODO             assert(cloned.view().key == spec_literal_int("1"));
// TODO             assert(cloned.view().value == spec_literal_int("2"));
// TODO
// TODO             let tracked m_3 = inst.tr1(&m_1);
// TODO             assert(m_3.view().value == spec_literal_int("4"));
// TODO
// TODO             let tracked m_5_12 = inst.tr2();
// TODO             assert(m_5_12.dom().contains(spec_literal_int("5")));
// TODO             assert(m_5_12.index(spec_literal_int("5")).view().value == spec_literal_int("9"));
// TODO             assert(m_5_12.dom().contains(spec_literal_int("12")));
// TODO             assert(m_5_12.index(spec_literal_int("12")).view().value == spec_literal_int("15"));
// TODO
// TODO             inst.tr3(&m_5_12);
// TODO         }
// TODO
// TODO         } // verus!
// TODO     } => Ok(())
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] pattern_binding_withdraw_assert_fails IMPORTS.to_string() + verus_code_str! {
// TODO         pub enum Goo {
// TODO             Bar,
// TODO             Qux(u64),
// TODO             Tal(u64, u64),
// TODO         }
// TODO
// TODO         tokenized_state_machine!{ Y {
// TODO             fields {
// TODO                 #[sharding(storage_map)]
// TODO                 pub storage_m: Map<int, Goo>,
// TODO
// TODO                 #[sharding(storage_option)]
// TODO                 pub storage_opt: Option<Goo>,
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr1() {
// TODO                     withdraw storage_opt -= Some(let Goo::Bar) by { // FAILS
// TODO                         assume(pre.storage_opt.is_Some());
// TODO                     };
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr2() {
// TODO                     withdraw storage_opt -= Some(let Goo::Qux(id1)) by { // FAILS
// TODO                         assume(pre.storage_opt.is_Some());
// TODO                     };
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr3() {
// TODO                     withdraw storage_opt -= Some(let Goo::Tal(id1, id2)) by { // FAILS
// TODO                         assume(pre.storage_opt.is_Some());
// TODO                     };
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr4() {
// TODO                     withdraw storage_m -= [1 => let Goo::Bar] by { // FAILS
// TODO                         assume(pre.storage_m.dom().contains(1));
// TODO                     };
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr5() {
// TODO                     withdraw storage_m -= [1 => let Goo::Qux(id1)] by { // FAILS
// TODO                         assume(pre.storage_m.dom().contains(1));
// TODO                     };
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr6() {
// TODO                     withdraw storage_m -= [1 => let Goo::Tal(id1, id2)] by { // FAILS
// TODO                         assume(pre.storage_m.dom().contains(1));
// TODO                     };
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_fails(e, 6)
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] special_refutable_pattern_binding_codegen IMPORTS.to_string() + verus_code_str! {
// TODO         pub enum Goo {
// TODO             Bar,
// TODO             Qux(u64),
// TODO             Tal(u64, u64),
// TODO         }
// TODO
// TODO         tokenized_state_machine!{ Y {
// TODO             fields {
// TODO                 #[sharding(map)]
// TODO                 pub m: Map<int, Goo>,
// TODO
// TODO                 #[sharding(storage_map)]
// TODO                 pub storage_m: Map<int, Goo>,
// TODO
// TODO                 #[sharding(option)]
// TODO                 pub opt: Option<Goo>,
// TODO
// TODO                 #[sharding(storage_option)]
// TODO                 pub storage_opt: Option<Goo>,
// TODO             }
// TODO
// TODO             init!{
// TODO                 initialize(m: Map<int, Goo>, opt: Option<Goo>) {
// TODO                     init m = m;
// TODO                     init storage_m = m;
// TODO                     init opt = opt;
// TODO                     init storage_opt = opt;
// TODO                 }
// TODO             }
// TODO
// TODO             #[inductive(initialize)]
// TODO             fn initialize_inductive(post: Self, m: Map<int, Goo>, opt: Option<Goo>) { }
// TODO
// TODO             transition!{
// TODO                 tr1() {
// TODO                     remove opt -= Some(let Goo::Bar);
// TODO                     withdraw storage_opt -= Some(let Goo::Bar);
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr2() {
// TODO                     remove opt -= Some(let Goo::Qux(i1));
// TODO                     withdraw storage_opt -= Some(let Goo::Qux(j1));
// TODO                     assert(i1 == j1);
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr3() {
// TODO                     remove opt -= Some(let Goo::Tal(i1, i2));
// TODO                     withdraw storage_opt -= Some(let Goo::Tal(j1, j2));
// TODO                     assert(i1 == j1);
// TODO                     assert(i2 == j2);
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr4(key: int) {
// TODO                     remove m -= [key => let Goo::Bar];
// TODO                     withdraw storage_m -= [key => let Goo::Bar];
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr5(key: int) {
// TODO                     remove m -= [key => let Goo::Qux(i1)];
// TODO                     withdraw storage_m -= [key => let Goo::Qux(j1)];
// TODO                     assert(i1 == j1);
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr6(key: int) {
// TODO                     remove m -= [key => let Goo::Tal(i1, i2)];
// TODO                     withdraw storage_m -= [key => let Goo::Tal(j1, j2)];
// TODO                     assert(i1 == j1);
// TODO                     assert(i2 == j2);
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr7(key: int) {
// TODO                     have opt >= Some(let Goo::Bar);
// TODO                     have m >= [key => let Goo::Bar];
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr8(key: int) {
// TODO                     have opt >= Some(let Goo::Qux(i1));
// TODO                     have m >= [key => let Goo::Qux(j1)];
// TODO                     require(i1 == j1);
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr9(key: int) {
// TODO                     have opt >= Some(let Goo::Tal(i1, i2));
// TODO                     have m >= [key => let Goo::Tal(j1, j2)];
// TODO                     require(i1 == j1);
// TODO                     require(i2 == j2);
// TODO                 }
// TODO             }
// TODO
// TODO             #[invariant]
// TODO             pub fn the_inv(&self) -> bool {
// TODO                 equal(self.m, self.storage_m)
// TODO                 && equal(self.opt, self.storage_opt)
// TODO             }
// TODO
// TODO                 #[inductive(tr1)]
// TODO                 fn tr1_inductive(pre: Self, post: Self) { }
// TODO
// TODO                 #[inductive(tr2)]
// TODO                 fn tr2_inductive(pre: Self, post: Self) { }
// TODO
// TODO                 #[inductive(tr3)]
// TODO                 fn tr3_inductive(pre: Self, post: Self) { }
// TODO
// TODO                 #[inductive(tr4)]
// TODO                 fn tr4_inductive(pre: Self, post: Self, key: int) { }
// TODO
// TODO                 #[inductive(tr5)]
// TODO                 fn tr5_inductive(pre: Self, post: Self, key: int) { }
// TODO
// TODO                 #[inductive(tr6)]
// TODO                 fn tr6_inductive(pre: Self, post: Self, key: int) { }
// TODO
// TODO                 #[inductive(tr7)]
// TODO                 fn tr7_inductive(pre: Self, post: Self, key: int) { }
// TODO
// TODO                 #[inductive(tr8)]
// TODO                 fn tr8_inductive(pre: Self, post: Self, key: int) { }
// TODO
// TODO                 #[inductive(tr9)]
// TODO                 fn tr9_inductive(pre: Self, post: Self, key: int) { }
// TODO         }}
// TODO
// TODO         verus! {
// TODO         spec fn rel_tr1(pre: Y::State, post: Y::State) -> bool {
// TODO             match pre.opt {
// TODO                 Option::Some(Goo::Bar) => {
// TODO                     match pre.storage_opt {
// TODO                         Option::Some(Goo::Bar) => {
// TODO                             equal(post.opt, Option::None)
// TODO                             && equal(post.storage_opt, Option::None)
// TODO                             && equal(post.m, pre.m)
// TODO                             && equal(post.storage_m, pre.storage_m)
// TODO                         }
// TODO                         _ => true,
// TODO                     }
// TODO                 }
// TODO                 _ => false,
// TODO             }
// TODO         }
// TODO
// TODO         spec fn rel_tr1_strong(pre: Y::State, post: Y::State) -> bool {
// TODO             match pre.opt {
// TODO                 Option::Some(Goo::Bar) => {
// TODO                     match pre.storage_opt {
// TODO                         Option::Some(Goo::Bar) => {
// TODO                             equal(post.opt, Option::None)
// TODO                             && equal(post.storage_opt, Option::None)
// TODO                             && equal(post.m, pre.m)
// TODO                             && equal(post.storage_m, pre.storage_m)
// TODO                         }
// TODO                         _ => false,
// TODO                     }
// TODO                 }
// TODO                 _ => false,
// TODO             }
// TODO         }
// TODO
// TODO         spec fn rel_tr2(pre: Y::State, post: Y::State) -> bool {
// TODO             match pre.opt {
// TODO                 Option::Some(Goo::Qux(i1)) => {
// TODO                     match pre.storage_opt {
// TODO                         Option::Some(Goo::Qux(j1)) => {
// TODO                             (i1 == j1) ==> {
// TODO                             &&& post.opt === Option::None
// TODO                             &&& post.storage_opt === Option::None
// TODO                             &&& post.m === pre.m
// TODO                             &&& post.storage_m === pre.storage_m
// TODO                             }
// TODO                         }
// TODO                         _ => true,
// TODO                     }
// TODO                 }
// TODO                 _ => false,
// TODO             }
// TODO         }
// TODO
// TODO         spec fn rel_tr2_strong(pre: Y::State, post: Y::State) -> bool {
// TODO             match pre.opt {
// TODO                 Option::Some(Goo::Qux(i1)) => {
// TODO                     match pre.storage_opt {
// TODO                         Option::Some(Goo::Qux(j1)) => {
// TODO                             &&& i1 == j1
// TODO                             &&& post.opt === Option::None
// TODO                             &&& post.storage_opt === Option::None
// TODO                             &&& post.m === pre.m
// TODO                             &&& post.storage_m === pre.storage_m
// TODO                         }
// TODO                         _ => false,
// TODO                     }
// TODO                 }
// TODO                 _ => false,
// TODO             }
// TODO         }
// TODO
// TODO         spec fn rel_tr3(pre: Y::State, post: Y::State) -> bool {
// TODO             match pre.opt {
// TODO                 Option::Some(Goo::Tal(i1, i2)) => {
// TODO                     match pre.storage_opt {
// TODO                         Option::Some(Goo::Tal(j1, j2)) => {
// TODO                             (i1 == j1 && i2 == j2) ==> {
// TODO                             &&& post.opt === Option::None
// TODO                             &&& post.storage_opt === Option::None
// TODO                             &&& post.m === pre.m
// TODO                             &&& post.storage_m === pre.storage_m
// TODO                             }
// TODO                         }
// TODO                         _ => true,
// TODO                     }
// TODO                 }
// TODO                 _ => false,
// TODO             }
// TODO         }
// TODO
// TODO         spec fn rel_tr3_strong(pre: Y::State, post: Y::State) -> bool {
// TODO             match pre.opt {
// TODO                 Option::Some(Goo::Tal(i1, i2)) => {
// TODO                     match pre.storage_opt {
// TODO                         Option::Some(Goo::Tal(j1, j2)) => {
// TODO                             &&& i1 == j1 && i2 == j2
// TODO                             &&& post.opt === Option::None
// TODO                             &&& post.storage_opt === Option::None
// TODO                             &&& post.m === pre.m
// TODO                             &&& post.storage_m === pre.storage_m
// TODO                         }
// TODO                         _ => false,
// TODO                     }
// TODO                 }
// TODO                 _ => false,
// TODO             }
// TODO         }
// TODO
// TODO         spec fn rel_tr4(pre: Y::State, post: Y::State, key: int) -> bool {
// TODO             pre.m.dom().contains(key)
// TODO             && match pre.m.index(key) {
// TODO                 Goo::Bar => {
// TODO                     pre.storage_m.dom().contains(key)
// TODO                     ==> match pre.storage_m.index(key) {
// TODO                         Goo::Bar => {
// TODO                             &&& post.opt === pre.opt
// TODO                             &&& post.storage_opt === pre.storage_opt
// TODO                             &&& post.m === pre.m.remove(key)
// TODO                             &&& post.storage_m === pre.storage_m.remove(key)
// TODO                         }
// TODO                         _ => true,
// TODO                     }
// TODO                 }
// TODO                 _ => false,
// TODO             }
// TODO         }
// TODO
// TODO         spec fn rel_tr4_strong(pre: Y::State, post: Y::State, key: int) -> bool {
// TODO             pre.m.dom().contains(key)
// TODO             && match pre.m.index(key) {
// TODO                 Goo::Bar => {
// TODO                     pre.storage_m.dom().contains(key)
// TODO                     && match pre.storage_m.index(key) {
// TODO                         Goo::Bar => {
// TODO                             &&& post.opt === pre.opt
// TODO                             &&& post.storage_opt === pre.storage_opt
// TODO                             &&& post.m === pre.m.remove(key)
// TODO                             &&& post.storage_m === pre.storage_m.remove(key)
// TODO                         }
// TODO                         _ => false,
// TODO                     }
// TODO                 }
// TODO                 _ => false,
// TODO             }
// TODO         }
// TODO
// TODO         spec fn rel_tr5(pre: Y::State, post: Y::State, key: int) -> bool {
// TODO             pre.m.dom().contains(key)
// TODO             && match pre.m.index(key) {
// TODO                 Goo::Qux(i1) => {
// TODO                     pre.storage_m.dom().contains(key)
// TODO                     ==> match pre.storage_m.index(key) {
// TODO                         Goo::Qux(j1) => {
// TODO                             (i1 == j1) ==> {
// TODO                             &&& post.opt === pre.opt
// TODO                             &&& post.storage_opt === pre.storage_opt
// TODO                             &&& post.m === pre.m.remove(key)
// TODO                             &&& post.storage_m === pre.storage_m.remove(key)
// TODO                             }
// TODO                         }
// TODO                         _ => true,
// TODO                     }
// TODO                 }
// TODO                 _ => false,
// TODO             }
// TODO         }
// TODO
// TODO         spec fn rel_tr5_strong(pre: Y::State, post: Y::State, key: int) -> bool {
// TODO             pre.m.dom().contains(key)
// TODO             && match pre.m.index(key) {
// TODO                 Goo::Qux(i1) => {
// TODO                     pre.storage_m.dom().contains(key)
// TODO                     && match pre.storage_m.index(key) {
// TODO                         Goo::Qux(j1) => {
// TODO                             &&& i1 == j1
// TODO                             &&& post.opt === pre.opt
// TODO                             &&& post.storage_opt === pre.storage_opt
// TODO                             &&& post.m === pre.m.remove(key)
// TODO                             &&& post.storage_m === pre.storage_m.remove(key)
// TODO                         }
// TODO                         _ => false,
// TODO                     }
// TODO                 }
// TODO                 _ => false,
// TODO             }
// TODO         }
// TODO
// TODO         spec fn rel_tr6(pre: Y::State, post: Y::State, key: int) -> bool {
// TODO             pre.m.dom().contains(key)
// TODO             && match pre.m.index(key) {
// TODO                 Goo::Tal(i1, i2) => {
// TODO                     pre.storage_m.dom().contains(key)
// TODO                     ==> match pre.storage_m.index(key) {
// TODO                         Goo::Tal(j1, j2) => {
// TODO                             (i1 == j1 && i2 == j2) ==> {
// TODO                             &&& post.opt === pre.opt
// TODO                             &&& post.storage_opt === pre.storage_opt
// TODO                             &&& post.m === pre.m.remove(key)
// TODO                             &&& post.storage_m === pre.storage_m.remove(key)
// TODO                             }
// TODO                         }
// TODO                         _ => true,
// TODO                     }
// TODO                 }
// TODO                 _ => false,
// TODO             }
// TODO         }
// TODO
// TODO         spec fn rel_tr6_strong(pre: Y::State, post: Y::State, key: int) -> bool {
// TODO             pre.m.dom().contains(key)
// TODO             && match pre.m.index(key) {
// TODO                 Goo::Tal(i1, i2) => {
// TODO                     pre.storage_m.dom().contains(key)
// TODO                     && match pre.storage_m.index(key) {
// TODO                         Goo::Tal(j1, j2) => {
// TODO                             i1 == j1 && i2 == j2
// TODO                             && equal(post.opt, pre.opt)
// TODO                             && equal(post.storage_opt, pre.storage_opt)
// TODO                             && equal(post.m, pre.m.remove(key))
// TODO                             && equal(post.storage_m, pre.storage_m.remove(key))
// TODO                         }
// TODO                         _ => false,
// TODO                     }
// TODO                 }
// TODO                 _ => false,
// TODO             }
// TODO         }
// TODO
// TODO         spec fn rel_tr7(pre: Y::State, post: Y::State, key: int) -> bool {
// TODO             match pre.opt {
// TODO                 Option::Some(Goo::Bar) => {
// TODO                     pre.m.dom().contains(key)
// TODO                     && match pre.m.index(key) {
// TODO                         Goo::Bar => equal(post, pre),
// TODO                         _ => false,
// TODO                     }
// TODO                 }
// TODO                 _ => false,
// TODO             }
// TODO         }
// TODO
// TODO         spec fn rel_tr7_strong(pre: Y::State, post: Y::State, key: int) -> bool {
// TODO             rel_tr7(pre, post, key)
// TODO         }
// TODO
// TODO         spec fn rel_tr8(pre: Y::State, post: Y::State, key: int) -> bool {
// TODO             match pre.opt {
// TODO                 Option::Some(Goo::Qux(i1)) => {
// TODO                     pre.m.dom().contains(key)
// TODO                     && match pre.m.index(key) {
// TODO                         Goo::Qux(j1) => i1 == j1 && equal(post, pre),
// TODO                         _ => false,
// TODO                     }
// TODO                 }
// TODO                 _ => false,
// TODO             }
// TODO         }
// TODO
// TODO         spec fn rel_tr8_strong(pre: Y::State, post: Y::State, key: int) -> bool {
// TODO             rel_tr8(pre, post, key)
// TODO         }
// TODO
// TODO         spec fn rel_tr9(pre: Y::State, post: Y::State, key: int) -> bool {
// TODO             match pre.opt {
// TODO                 Option::Some(Goo::Tal(i1, i2)) => {
// TODO                     pre.m.dom().contains(key)
// TODO                     && match pre.m.index(key) {
// TODO                         Goo::Tal(j1, j2) => i1 == j1 && i2 == j2 && equal(post, pre),
// TODO                         _ => false,
// TODO                     }
// TODO                 }
// TODO                 _ => false,
// TODO             }
// TODO         }
// TODO
// TODO         spec fn rel_tr9_strong(pre: Y::State, post: Y::State, key: int) -> bool {
// TODO             rel_tr9(pre, post, key)
// TODO         }
// TODO
// TODO         proof fn correct_tr(pre: Y::State, post: Y::State, key: int) {
// TODO           ensures([
// TODO               rel_tr1(pre, post) == Y::State::tr1(pre, post),
// TODO               rel_tr1_strong(pre, post) == Y::State::tr1_strong(pre, post),
// TODO               rel_tr2(pre, post) == Y::State::tr2(pre, post),
// TODO               rel_tr2_strong(pre, post) == Y::State::tr2_strong(pre, post),
// TODO               rel_tr3(pre, post) == Y::State::tr3(pre, post),
// TODO               rel_tr3_strong(pre, post) == Y::State::tr3_strong(pre, post),
// TODO               rel_tr4(pre, post, key) == Y::State::tr4(pre, post, key),
// TODO               rel_tr4_strong(pre, post, key) == Y::State::tr4_strong(pre, post, key),
// TODO               rel_tr5(pre, post, key) == Y::State::tr5(pre, post, key),
// TODO               rel_tr5_strong(pre, post, key) == Y::State::tr5_strong(pre, post, key),
// TODO               rel_tr6(pre, post, key) == Y::State::tr6(pre, post, key),
// TODO               rel_tr6_strong(pre, post, key) == Y::State::tr6_strong(pre, post, key),
// TODO               rel_tr7(pre, post, key) == Y::State::tr7(pre, post, key),
// TODO               rel_tr7_strong(pre, post, key) == Y::State::tr7_strong(pre, post, key),
// TODO               rel_tr8(pre, post, key) == Y::State::tr8(pre, post, key),
// TODO               rel_tr8_strong(pre, post, key) == Y::State::tr8_strong(pre, post, key),
// TODO               rel_tr9(pre, post, key) == Y::State::tr9(pre, post, key),
// TODO               rel_tr9_strong(pre, post, key) == Y::State::tr9_strong(pre, post, key),
// TODO           ]);
// TODO         }
// TODO
// TODO         proof fn test_inst1() {
// TODO             let tracked mut p_m = Map::tracked_empty();
// TODO             p_m.tracked_insert(spec_literal_int("1"), Goo::Bar);
// TODO
// TODO             let tracked (Tracked(inst), Tracked(mut m_token), Tracked(opt_token)) = Y::Instance::initialize(
// TODO                 map![spec_literal_int("1") => Goo::Bar],
// TODO                 Option::Some(Goo::Bar),
// TODO                 p_m,
// TODO                 Option::Some(Goo::Bar),
// TODO             );
// TODO
// TODO             assert(m_token.dom().contains(spec_literal_int("1")));
// TODO             let tracked kv = m_token.tracked_remove(spec_literal_int("1"));
// TODO             let tracked o = match opt_token {
// TODO                 Option::None => proof_from_false(),
// TODO                 Option::Some(t) => t,
// TODO             };
// TODO
// TODO             inst.tr7(spec_literal_int("1"), &kv, &o);
// TODO
// TODO             let tracked wi = inst.tr1(o);
// TODO             assert(equal(wi, Goo::Bar));
// TODO
// TODO             let tracked wi2 = inst.tr4(spec_literal_int("1"), kv);
// TODO             assert(equal(wi2, Goo::Bar));
// TODO         }
// TODO
// TODO         proof fn test_inst2() {
// TODO             let tracked mut p_m = Map::tracked_empty();
// TODO             p_m.tracked_insert(spec_literal_int("1"), Goo::Qux(8u64));
// TODO
// TODO             let tracked (Tracked(inst), Tracked(mut m_token), Tracked(opt_token)) = Y::Instance::initialize(
// TODO                 map![spec_literal_int("1") => Goo::Qux(8u64)],
// TODO                 Option::Some(Goo::Qux(8u64)),
// TODO                 p_m,
// TODO                 Option::Some(Goo::Qux(8u64)),
// TODO             );
// TODO
// TODO             assert(m_token.dom().contains(spec_literal_int("1")));
// TODO             let tracked kv = m_token.tracked_remove(spec_literal_int("1"));
// TODO             let tracked o = match opt_token {
// TODO                 Option::None => proof_from_false(),
// TODO                 Option::Some(t) => t,
// TODO             };
// TODO
// TODO             inst.tr8(spec_literal_int("1"), &kv, &o);
// TODO
// TODO             let tracked wi = inst.tr2(o);
// TODO             assert(equal(wi, Goo::Qux(8u64)));
// TODO
// TODO             let tracked wi2 = inst.tr5(spec_literal_int("1"), kv);
// TODO             assert(equal(wi2, Goo::Qux(8u64)));
// TODO         }
// TODO
// TODO         proof fn test_inst3() {
// TODO             let tracked mut p_m = Map::tracked_empty();
// TODO             p_m.tracked_insert(spec_literal_int("1"), Goo::Tal(8u64, 9u64));
// TODO
// TODO             let tracked (Tracked(inst), Tracked(mut m_token), Tracked(opt_token)) = Y::Instance::initialize(
// TODO                 map![spec_literal_int("1") => Goo::Tal(8u64, 9u64)],
// TODO                 Option::Some(Goo::Tal(8u64, 9u64)),
// TODO                 p_m,
// TODO                 Option::Some(Goo::Tal(8u64, 9u64)),
// TODO             );
// TODO
// TODO             assert(m_token.dom().contains(spec_literal_int("1")));
// TODO             let tracked kv = m_token.tracked_remove(spec_literal_int("1"));
// TODO             let tracked o = match opt_token {
// TODO                 Option::None => proof_from_false(),
// TODO                 Option::Some(t) => t,
// TODO             };
// TODO
// TODO             inst.tr9(spec_literal_int("1"), &kv, &o);
// TODO
// TODO             let tracked wi = inst.tr3(o);
// TODO             assert(equal(wi, Goo::Tal(8u64, 9u64)));
// TODO
// TODO             let tracked wi2 = inst.tr6(spec_literal_int("1"), kv);
// TODO             assert(equal(wi2, Goo::Tal(8u64, 9u64)));
// TODO         }
// TODO
// TODO         proof fn test_precondition_remove1(tracked inst: Y::Instance, tracked t: Y::opt)
// TODO         {
// TODO           requires(equal(t.view().instance, inst));
// TODO           let tracked k = inst.tr1(t); // FAILS
// TODO         }
// TODO
// TODO         proof fn test_precondition_remove2(tracked inst: Y::Instance, tracked t: Y::opt)
// TODO         {
// TODO           requires(equal(t.view().instance, inst));
// TODO           let tracked k = inst.tr2(t); // FAILS
// TODO         }
// TODO
// TODO         proof fn test_precondition_remove3(tracked inst: Y::Instance, tracked t: Y::opt)
// TODO         {
// TODO           requires(equal(t.view().instance, inst));
// TODO           let tracked k = inst.tr3(t); // FAILS
// TODO         }
// TODO
// TODO         proof fn test_precondition_map_remove1(tracked inst: Y::Instance, tracked t: Y::m)
// TODO         {
// TODO           requires(equal(t.view().instance, inst) && t.view().key == spec_literal_int("1"));
// TODO           let tracked k = inst.tr4(spec_literal_int("1"), t); // FAILS
// TODO         }
// TODO
// TODO         proof fn test_precondition_map_remove2(tracked inst: Y::Instance, tracked t: Y::m)
// TODO         {
// TODO           requires(equal(t.view().instance, inst) && t.view().key == spec_literal_int("1"));
// TODO           let tracked k = inst.tr5(spec_literal_int("1"), t); // FAILS
// TODO         }
// TODO
// TODO         proof fn test_precondition_map_remove3(tracked inst: Y::Instance, tracked t: Y::m)
// TODO         {
// TODO           requires(equal(t.view().instance, inst) && t.view().key == spec_literal_int("1"));
// TODO           let tracked k = inst.tr6(spec_literal_int("1"), t); // FAILS
// TODO         }
// TODO
// TODO         proof fn test_precondition_have1(tracked inst: Y::Instance, tracked t: Y::opt, tracked u: Y::m)
// TODO         {
// TODO           requires(equal(t.view().instance, inst) && equal(u.view().instance, inst) && u.view().key == spec_literal_int("1")
// TODO               && equal(t.view().value, Goo::Bar)
// TODO           );
// TODO           let tracked k = inst.tr7(spec_literal_int("1"), &u, &t); // FAILS
// TODO         }
// TODO
// TODO         proof fn test_precondition_have2(tracked inst: Y::Instance, tracked t: Y::opt, tracked u: Y::m)
// TODO         {
// TODO           requires(equal(t.view().instance, inst) && equal(u.view().instance, inst) && u.view().key == spec_literal_int("1")
// TODO               && equal(u.view().value, Goo::Bar)
// TODO           );
// TODO           let tracked k = inst.tr7(spec_literal_int("1"), &u, &t); // FAILS
// TODO         }
// TODO
// TODO         proof fn test_precondition_have3(tracked inst: Y::Instance, tracked t: Y::opt, tracked u: Y::m)
// TODO         {
// TODO           requires(equal(t.view().instance, inst) && equal(u.view().instance, inst) && u.view().key == spec_literal_int("1")
// TODO               && equal(u.view().value, t.view().value));
// TODO           let tracked k = inst.tr8(spec_literal_int("1"), &u, &t); // FAILS
// TODO         }
// TODO
// TODO         proof fn test_precondition_have4(tracked inst: Y::Instance, tracked t: Y::opt, tracked u: Y::m)
// TODO         {
// TODO           requires(equal(t.view().instance, inst) && equal(u.view().instance, inst) && u.view().key == spec_literal_int("1")
// TODO               && equal(u.view().value, t.view().value));
// TODO           let k = inst.tr9(1, &u, &t); // FAILS
// TODO         }
// TODO
// TODO         }
// TODO     } => Err(e) => assert_fails(e, 10)
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] labels_wrong_type_name IMPORTS.to_string() + verus_code_str! {
// TODO         state_machine!{ Y {
// TODO             fields {
// TODO                 pub x: int,
// TODO             }
// TODO
// TODO             pub struct AsdfWeirdName { }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "only supports the declaration of a `Label` and `InitLabel` types")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] labels_init_missing IMPORTS.to_string() + verus_code_str! {
// TODO         state_machine!{ Y {
// TODO             fields {
// TODO                 pub x: int,
// TODO             }
// TODO
// TODO             pub struct Label { }
// TODO             pub struct InitLabel { }
// TODO
// TODO             init!{
// TODO                 tr() {
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "the first param to an 'init'")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] labels_init_missing2 IMPORTS.to_string() + verus_code_str! {
// TODO         state_machine!{ Y {
// TODO             fields {
// TODO                 pub x: int,
// TODO             }
// TODO
// TODO             pub struct Label { }
// TODO             pub struct InitLabel { }
// TODO
// TODO             init!{
// TODO                 tr(x: int) {
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "the first param to an 'init'")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] labels_tr_missing IMPORTS.to_string() + verus_code_str! {
// TODO         state_machine!{ Y {
// TODO             fields {
// TODO                 pub x: int,
// TODO             }
// TODO
// TODO             pub struct Label { }
// TODO             pub struct InitLabel { }
// TODO
// TODO             transition!{
// TODO                 tr(x: int) {
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "the first param to a 'transition'")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] labels_readonly_missing IMPORTS.to_string() + verus_code_str! {
// TODO         state_machine!{ Y {
// TODO             fields {
// TODO                 pub x: int,
// TODO             }
// TODO
// TODO             pub struct Label { }
// TODO             pub struct InitLabel { }
// TODO
// TODO             readonly!{
// TODO                 tr(x: int) {
// TODO                 }
// TODO             }
// TODO         }}
// TODO     } => Err(e) => assert_error_msg(e, "the first param to a 'readonly'")
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] bool_codegen IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ Y {
// TODO             fields {
// TODO                 #[sharding(bool)]
// TODO                 pub b: bool,
// TODO             }
// TODO
// TODO             init!{
// TODO                 init_false() {
// TODO                     init b = false;
// TODO                 }
// TODO             }
// TODO
// TODO             init!{
// TODO                 init_true() {
// TODO                     init b = true;
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr_add() {
// TODO                     add b += true by {
// TODO                         assert(pre.b === false); // FAILS
// TODO                     };
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr_have() {
// TODO                     have b >= true;
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr_remove() {
// TODO                     remove b -= true;
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr_add_gen(x: bool) {
// TODO                     add b += (x) by {
// TODO                         assert(pre.b === false || x === false); // FAILS
// TODO                     };
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr_have_gen(x: bool) {
// TODO                     have b >= (x);
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr_remove_gen(x: bool) {
// TODO                     remove b -= (x);
// TODO                 }
// TODO             }
// TODO         }}
// TODO
// TODO         verus!{
// TODO
// TODO         spec fn rel_tr1(pre: Y::State, post: Y::State) -> bool {
// TODO             pre.b === false ==> post.b === true
// TODO         }
// TODO
// TODO         spec fn rel_tr1_strong(pre: Y::State, post: Y::State) -> bool {
// TODO             pre.b === false && post.b === true
// TODO         }
// TODO
// TODO         spec fn rel_tr2(pre: Y::State, post: Y::State) -> bool {
// TODO             pre.b === true && post.b === true
// TODO         }
// TODO
// TODO         spec fn rel_tr2_strong(pre: Y::State, post: Y::State) -> bool {
// TODO             pre.b === true && post.b === true
// TODO         }
// TODO
// TODO         spec fn rel_tr3(pre: Y::State, post: Y::State) -> bool {
// TODO             pre.b === true && post.b === false
// TODO         }
// TODO
// TODO         spec fn rel_tr3_strong(pre: Y::State, post: Y::State) -> bool {
// TODO             pre.b === true && post.b === false
// TODO         }
// TODO
// TODO         spec fn rel_tr4(pre: Y::State, post: Y::State, x: bool) -> bool {
// TODO             (!pre.b || !x) ==> (post.b === (pre.b || x))
// TODO         }
// TODO
// TODO         spec fn rel_tr4_strong(pre: Y::State, post: Y::State, x: bool) -> bool {
// TODO             (!pre.b || !x) && (post.b === (pre.b || x))
// TODO         }
// TODO
// TODO         spec fn rel_tr5(pre: Y::State, post: Y::State, x: bool) -> bool {
// TODO             (x ==> pre.b) && (post.b === pre.b)
// TODO         }
// TODO
// TODO         spec fn rel_tr5_strong(pre: Y::State, post: Y::State, x: bool) -> bool {
// TODO             (x ==> pre.b) && (post.b === pre.b)
// TODO         }
// TODO
// TODO         spec fn rel_tr6(pre: Y::State, post: Y::State, x: bool) -> bool {
// TODO             (x ==> pre.b) && (post.b === (pre.b && !x))
// TODO         }
// TODO
// TODO         spec fn rel_tr6_strong(pre: Y::State, post: Y::State, x: bool) -> bool {
// TODO             (x ==> pre.b) && (post.b === (pre.b && !x))
// TODO         }
// TODO
// TODO         proof fn correct_tr(pre: Y::State, post: Y::State, x: bool) {
// TODO             ensures([
// TODO                 rel_tr1(pre, post) == Y::State::tr_add(pre, post),
// TODO                 rel_tr1_strong(pre, post) == Y::State::tr_add_strong(pre, post),
// TODO                 rel_tr2(pre, post) == Y::State::tr_have(pre, post),
// TODO                 rel_tr2_strong(pre, post) == Y::State::tr_have_strong(pre, post),
// TODO                 rel_tr3(pre, post) == Y::State::tr_remove(pre, post),
// TODO                 rel_tr3_strong(pre, post) == Y::State::tr_remove_strong(pre, post),
// TODO
// TODO                 rel_tr4(pre, post, x) == Y::State::tr_add_gen(pre, post, x),
// TODO                 rel_tr4_strong(pre, post, x) == Y::State::tr_add_gen_strong(pre, post, x),
// TODO                 rel_tr5(pre, post, x) == Y::State::tr_have_gen(pre, post, x),
// TODO                 rel_tr5_strong(pre, post, x) == Y::State::tr_have_gen_strong(pre, post, x),
// TODO                 rel_tr6(pre, post, x) == Y::State::tr_remove_gen(pre, post, x),
// TODO                 rel_tr6_strong(pre, post, x) == Y::State::tr_remove_gen_strong(pre, post, x),
// TODO             ]);
// TODO         }
// TODO
// TODO         proof fn test_inst1() {
// TODO             let tracked (Tracked(inst), Tracked(token_f)) = Y::Instance::init_false();
// TODO             assert(token_f.is_None());
// TODO
// TODO             let tracked tok = inst.tr_add();
// TODO             assert(equal(tok.view().instance, inst));
// TODO             inst.tr_have(&tok);
// TODO             inst.tr_remove(tok);
// TODO
// TODO             let tracked opt_tok = inst.tr_add_gen(true);
// TODO             assert(opt_tok.is_Some());
// TODO             assert(equal(opt_tok.get_Some_0().view().instance, inst));
// TODO             inst.tr_have_gen(true, &opt_tok);
// TODO             inst.tr_remove_gen(true, opt_tok);
// TODO
// TODO             let tracked opt_tok = inst.tr_add_gen(false);
// TODO             assert(opt_tok.is_None());
// TODO             inst.tr_have_gen(false, &opt_tok);
// TODO             inst.tr_remove_gen(false, opt_tok);
// TODO         }
// TODO
// TODO         proof fn test_inst1_fail() {
// TODO             let tracked (Tracked(inst), Tracked(token_f)) = Y::Instance::init_false();
// TODO             assert(token_f.is_None());
// TODO
// TODO             let tracked opt_tok = inst.tr_add_gen(false);
// TODO             assert(opt_tok.is_None());
// TODO             inst.tr_have_gen(true, &opt_tok);   // FAILS
// TODO         }
// TODO
// TODO         proof fn test_inst2() {
// TODO             let tracked (Tracked(inst), Tracked(token_t)) = Y::Instance::init_true();
// TODO             assert(token_t.is_Some());
// TODO             assert(equal(token_t.get_Some_0().view().instance, inst));
// TODO         }
// TODO
// TODO         }
// TODO     } => Err(e) => assert_fails(e, 3)
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] persistent_bool_codegen IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ Y {
// TODO             fields {
// TODO                 #[sharding(persistent_bool)]
// TODO                 pub b: bool,
// TODO             }
// TODO
// TODO             init!{
// TODO                 init_false() {
// TODO                     init b = false;
// TODO                 }
// TODO             }
// TODO
// TODO             init!{
// TODO                 init_true() {
// TODO                     init b = true;
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr_add() {
// TODO                     add b (union)= true;
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr_have() {
// TODO                     have b >= true;
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr_add_gen(x: bool) {
// TODO                     add b (union)= (x);
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr_have_gen(x: bool) {
// TODO                     have b >= (x);
// TODO                 }
// TODO             }
// TODO         }}
// TODO
// TODO         verus!{
// TODO
// TODO         spec fn rel_tr1(pre: Y::State, post: Y::State) -> bool {
// TODO             post.b === true
// TODO         }
// TODO
// TODO         spec fn rel_tr1_strong(pre: Y::State, post: Y::State) -> bool {
// TODO             post.b === true
// TODO         }
// TODO
// TODO         spec fn rel_tr2(pre: Y::State, post: Y::State) -> bool {
// TODO             pre.b === true && post.b === true
// TODO         }
// TODO
// TODO         spec fn rel_tr2_strong(pre: Y::State, post: Y::State) -> bool {
// TODO             pre.b === true && post.b === true
// TODO         }
// TODO
// TODO         spec fn rel_tr4(pre: Y::State, post: Y::State, x: bool) -> bool {
// TODO             (post.b === (pre.b || x))
// TODO         }
// TODO
// TODO         spec fn rel_tr4_strong(pre: Y::State, post: Y::State, x: bool) -> bool {
// TODO             (post.b === (pre.b || x))
// TODO         }
// TODO
// TODO         spec fn rel_tr5(pre: Y::State, post: Y::State, x: bool) -> bool {
// TODO             (x ==> pre.b) && (post.b === pre.b)
// TODO         }
// TODO
// TODO         spec fn rel_tr5_strong(pre: Y::State, post: Y::State, x: bool) -> bool {
// TODO             (x ==> pre.b) && (post.b === pre.b)
// TODO         }
// TODO
// TODO         proof fn correct_tr(pre: Y::State, post: Y::State, x: bool) {
// TODO             ensures([
// TODO                 rel_tr1(pre, post) == Y::State::tr_add(pre, post),
// TODO                 rel_tr1_strong(pre, post) == Y::State::tr_add_strong(pre, post),
// TODO                 rel_tr2(pre, post) == Y::State::tr_have(pre, post),
// TODO                 rel_tr2_strong(pre, post) == Y::State::tr_have_strong(pre, post),
// TODO
// TODO                 rel_tr4(pre, post, x) == Y::State::tr_add_gen(pre, post, x),
// TODO                 rel_tr4_strong(pre, post, x) == Y::State::tr_add_gen_strong(pre, post, x),
// TODO                 rel_tr5(pre, post, x) == Y::State::tr_have_gen(pre, post, x),
// TODO                 rel_tr5_strong(pre, post, x) == Y::State::tr_have_gen_strong(pre, post, x),
// TODO             ]);
// TODO         }
// TODO
// TODO         proof fn test_inst1() {
// TODO             let tracked (Tracked(inst), Tracked(token_f)) = Y::Instance::init_false();
// TODO             assert(token_f.is_None());
// TODO
// TODO             let tracked tok = inst.tr_add();
// TODO             assert(equal(tok.view().instance, inst));
// TODO             inst.tr_have(&tok);
// TODO
// TODO             let tracked tok1 = tok.clone();
// TODO             assert(equal(tok, tok1));
// TODO
// TODO             let tracked opt_tok = inst.tr_add_gen(true);
// TODO             assert(opt_tok.is_Some());
// TODO             assert(equal(opt_tok.get_Some_0().view().instance, inst));
// TODO             inst.tr_have_gen(true, &opt_tok);
// TODO
// TODO             let tracked opt_tok = inst.tr_add_gen(false);
// TODO             assert(opt_tok.is_None());
// TODO             inst.tr_have_gen(false, &opt_tok);
// TODO         }
// TODO
// TODO         proof fn test_inst1_fail() {
// TODO             let tracked (Tracked(inst), Tracked(token_f)) = Y::Instance::init_false();
// TODO             assert(token_f.is_None());
// TODO
// TODO             let tracked opt_tok = inst.tr_add_gen(false);
// TODO             assert(opt_tok.is_None());
// TODO             inst.tr_have_gen(true, &opt_tok);   // FAILS
// TODO         }
// TODO
// TODO         proof fn test_inst2() {
// TODO             let tracked (Tracked(inst), Tracked(token_t)) = Y::Instance::init_true();
// TODO             assert(token_t.is_Some());
// TODO             assert(equal(token_t.get_Some_0().view().instance, inst));
// TODO         }
// TODO
// TODO         }
// TODO     } => Err(e) => assert_fails(e, 1)
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] persistent_count_codegen IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ Y {
// TODO             fields {
// TODO                 #[sharding(persistent_count)]
// TODO                 pub c: nat,
// TODO             }
// TODO
// TODO             init!{
// TODO                 initialize() {
// TODO                     init c = 9;
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr_add() {
// TODO                     add c (union)= (2);
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr_have() {
// TODO                     have c >= (2);
// TODO                 }
// TODO             }
// TODO         }}
// TODO
// TODO         verus!{
// TODO
// TODO         spec fn rel_tr1(pre: Y::State, post: Y::State) -> bool {
// TODO             post.c == if pre.c <= 2 { 2 } else { pre.c }
// TODO         }
// TODO
// TODO         spec fn rel_tr1_strong(pre: Y::State, post: Y::State) -> bool {
// TODO             post.c == if pre.c <= 2 { 2 } else { pre.c }
// TODO         }
// TODO
// TODO         spec fn rel_tr2(pre: Y::State, post: Y::State) -> bool {
// TODO             pre.c >= 2 && post.c == pre.c
// TODO         }
// TODO
// TODO         spec fn rel_tr2_strong(pre: Y::State, post: Y::State) -> bool {
// TODO             pre.c >= 2 && post.c == pre.c
// TODO         }
// TODO
// TODO         proof fn correct_tr(pre: Y::State, post: Y::State) {
// TODO             ensures([
// TODO                 rel_tr1(pre, post) == Y::State::tr_add(pre, post),
// TODO                 rel_tr1_strong(pre, post) == Y::State::tr_add_strong(pre, post),
// TODO                 rel_tr2(pre, post) == Y::State::tr_have(pre, post),
// TODO                 rel_tr2_strong(pre, post) == Y::State::tr_have_strong(pre, post),
// TODO             ]);
// TODO         }
// TODO
// TODO         proof fn test_inst() {
// TODO             let tracked (Tracked(inst), Tracked(t1)) = Y::Instance::initialize();
// TODO             assert(t1.view().count == spec_literal_nat("9"));
// TODO
// TODO             let tracked t2 = t1.weaken(spec_literal_nat("2"));
// TODO
// TODO             inst.tr_have(&t2);
// TODO
// TODO             let tracked t4 = inst.tr_add();
// TODO             assert(t4.view().count == spec_literal_nat("2"));
// TODO
// TODO             let tracked t2_clone = t2.clone();
// TODO             assert(equal(t2, t2_clone));
// TODO         }
// TODO
// TODO         proof fn test_weaken_fail() {
// TODO             let tracked (Tracked(inst), Tracked(t1)) = Y::Instance::initialize();
// TODO             let tracked t2 = t1.weaken(spec_literal_nat("800")); // FAILS
// TODO         }
// TODO
// TODO         }
// TODO     } => Err(e) => assert_fails(e, 1)
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] set_codegen IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ Y {
// TODO             fields {
// TODO                 #[sharding(set)]
// TODO                 pub b: Set<int>,
// TODO             }
// TODO
// TODO             init!{
// TODO                 initialize() {
// TODO                     init b = Set::empty().insert(19);
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr_add() {
// TODO                     add b += set { 5 }; // FAILS
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr_have() {
// TODO                     have b >= set { 5 };
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr_remove() {
// TODO                     remove b -= set { 5 };
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr_add_gen() {
// TODO                     add b += (Set::empty().insert(6)); // FAILS
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr_have_gen() {
// TODO                     have b >= (Set::empty().insert(6));
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr_remove_gen() {
// TODO                     remove b -= (Set::empty().insert(6));
// TODO                 }
// TODO             }
// TODO         }}
// TODO
// TODO         verus!{
// TODO
// TODO         spec fn rel_tr1(pre: Y::State, post: Y::State) -> bool {
// TODO             !pre.b.contains(5)
// TODO             ==> post.b === pre.b.insert(5)
// TODO         }
// TODO
// TODO         spec fn rel_tr1_strong(pre: Y::State, post: Y::State) -> bool {
// TODO             !pre.b.contains(5)
// TODO             && post.b === pre.b.insert(5)
// TODO         }
// TODO
// TODO         spec fn rel_tr2(pre: Y::State, post: Y::State) -> bool {
// TODO             pre.b.contains(5)
// TODO             && pre.b === post.b
// TODO         }
// TODO
// TODO         spec fn rel_tr2_strong(pre: Y::State, post: Y::State) -> bool {
// TODO             pre.b.contains(5)
// TODO             && pre.b === post.b
// TODO         }
// TODO
// TODO         spec fn rel_tr3(pre: Y::State, post: Y::State) -> bool {
// TODO             pre.b.contains(5)
// TODO             && post.b === pre.b.remove(5)
// TODO         }
// TODO
// TODO         spec fn rel_tr3_strong(pre: Y::State, post: Y::State) -> bool {
// TODO             pre.b.contains(5)
// TODO             && post.b === pre.b.remove(5)
// TODO         }
// TODO
// TODO         spec fn rel_tr4(pre: Y::State, post: Y::State) -> bool {
// TODO             !pre.b.contains(6)
// TODO             ==> post.b === pre.b.union(Set::empty().insert(6))
// TODO         }
// TODO
// TODO         spec fn rel_tr4_strong(pre: Y::State, post: Y::State) -> bool {
// TODO             !pre.b.contains(6)
// TODO             && post.b === pre.b.union(Set::empty().insert(6))
// TODO         }
// TODO
// TODO         spec fn rel_tr5(pre: Y::State, post: Y::State) -> bool {
// TODO             pre.b.contains(6)
// TODO             && pre.b === post.b
// TODO         }
// TODO
// TODO         spec fn rel_tr5_strong(pre: Y::State, post: Y::State) -> bool {
// TODO             pre.b.contains(6)
// TODO             && pre.b === post.b
// TODO         }
// TODO
// TODO         spec fn rel_tr6(pre: Y::State, post: Y::State) -> bool {
// TODO             pre.b.contains(6)
// TODO             && post.b === pre.b.difference(Set::empty().insert(6))
// TODO         }
// TODO
// TODO         spec fn rel_tr6_strong(pre: Y::State, post: Y::State) -> bool {
// TODO             pre.b.contains(6)
// TODO             && post.b === pre.b.difference(Set::empty().insert(6))
// TODO         }
// TODO
// TODO         proof fn correct_tr(pre: Y::State, post: Y::State) {
// TODO             ensures([
// TODO                 rel_tr1(pre, post) == Y::State::tr_add(pre, post),
// TODO                 rel_tr1_strong(pre, post) == Y::State::tr_add_strong(pre, post),
// TODO                 rel_tr2(pre, post) == Y::State::tr_have(pre, post),
// TODO                 rel_tr2_strong(pre, post) == Y::State::tr_have_strong(pre, post),
// TODO                 rel_tr3(pre, post) == Y::State::tr_remove(pre, post),
// TODO                 rel_tr3_strong(pre, post) == Y::State::tr_remove_strong(pre, post),
// TODO
// TODO                 rel_tr4(pre, post) == Y::State::tr_add_gen(pre, post),
// TODO                 rel_tr4_strong(pre, post) == Y::State::tr_add_gen_strong(pre, post),
// TODO                 rel_tr5(pre, post) == Y::State::tr_have_gen(pre, post),
// TODO                 rel_tr5_strong(pre, post) == Y::State::tr_have_gen_strong(pre, post),
// TODO                 rel_tr6(pre, post) == Y::State::tr_remove_gen(pre, post),
// TODO                 rel_tr6_strong(pre, post) == Y::State::tr_remove_gen_strong(pre, post),
// TODO             ]);
// TODO         }
// TODO
// TODO         proof fn test_inst1() {
// TODO             let tracked (Tracked(inst), Tracked(token_f)) = Y::Instance::initialize();
// TODO             assert(Set::empty().insert(spec_literal_int("19")).contains(spec_literal_int("19")));
// TODO             assert(token_f.dom().contains(spec_literal_int("19")));
// TODO             assert(equal(token_f.index(spec_literal_int("19")).view(), Y::token![
// TODO                 inst => b => spec_literal_int("19")
// TODO             ]));
// TODO
// TODO             let tracked token1 = inst.tr_add();
// TODO             assert(equal(token1.view().instance, inst));
// TODO             assert(token1.view().key == spec_literal_int("5"));
// TODO             inst.tr_have(&token1);
// TODO             inst.tr_remove(token1);
// TODO
// TODO             let tracked token_set = inst.tr_add_gen();
// TODO             assert(Set::empty().insert(spec_literal_int("6")).contains(spec_literal_int("6")));
// TODO             assert(token_set.dom().contains(spec_literal_int("6")));
// TODO             assert(equal(token_set.index(spec_literal_int("6")).view(), Y::token![
// TODO                 inst => b => spec_literal_int("6")
// TODO             ]));
// TODO             inst.tr_have_gen(&token_set);
// TODO             inst.tr_remove_gen(token_set);
// TODO         }
// TODO
// TODO         }
// TODO     } => Err(e) => assert_fails(e, 2)
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] persistent_set_codegen IMPORTS.to_string() + verus_code_str! {
// TODO         tokenized_state_machine!{ Y {
// TODO             fields {
// TODO                 #[sharding(persistent_set)]
// TODO                 pub b: Set<int>,
// TODO             }
// TODO
// TODO             init!{
// TODO                 initialize() {
// TODO                     init b = Set::empty().insert(19);
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr_add() {
// TODO                     add b (union)= set { 5 };
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr_have() {
// TODO                     have b >= set { 5 };
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr_add_gen() {
// TODO                     add b (union)= (Set::empty().insert(6));
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 tr_have_gen() {
// TODO                     have b >= (Set::empty().insert(6));
// TODO                 }
// TODO             }
// TODO         }}
// TODO
// TODO         verus!{
// TODO
// TODO         spec fn rel_tr1(pre: Y::State, post: Y::State) -> bool {
// TODO             post.b === pre.b.insert(5)
// TODO         }
// TODO
// TODO         spec fn rel_tr1_strong(pre: Y::State, post: Y::State) -> bool {
// TODO             post.b === pre.b.insert(5)
// TODO         }
// TODO
// TODO         spec fn rel_tr2(pre: Y::State, post: Y::State) -> bool {
// TODO             pre.b.contains(5)
// TODO             && pre.b === post.b
// TODO         }
// TODO
// TODO         spec fn rel_tr2_strong(pre: Y::State, post: Y::State) -> bool {
// TODO             pre.b.contains(5)
// TODO             && pre.b === post.b
// TODO         }
// TODO
// TODO         spec fn rel_tr4(pre: Y::State, post: Y::State) -> bool {
// TODO             post.b === pre.b.union(Set::empty().insert(6))
// TODO         }
// TODO
// TODO         spec fn rel_tr4_strong(pre: Y::State, post: Y::State) -> bool {
// TODO             post.b === pre.b.union(Set::empty().insert(6))
// TODO         }
// TODO
// TODO         spec fn rel_tr5(pre: Y::State, post: Y::State) -> bool {
// TODO             pre.b.contains(6)
// TODO             && pre.b === post.b
// TODO         }
// TODO
// TODO         spec fn rel_tr5_strong(pre: Y::State, post: Y::State) -> bool {
// TODO             pre.b.contains(6)
// TODO             && pre.b === post.b
// TODO         }
// TODO
// TODO         proof fn correct_tr(pre: Y::State, post: Y::State) {
// TODO             ensures([
// TODO                 rel_tr1(pre, post) == Y::State::tr_add(pre, post),
// TODO                 rel_tr1_strong(pre, post) == Y::State::tr_add_strong(pre, post),
// TODO                 rel_tr2(pre, post) == Y::State::tr_have(pre, post),
// TODO                 rel_tr2_strong(pre, post) == Y::State::tr_have_strong(pre, post),
// TODO
// TODO                 rel_tr4(pre, post) == Y::State::tr_add_gen(pre, post),
// TODO                 rel_tr4_strong(pre, post) == Y::State::tr_add_gen_strong(pre, post),
// TODO                 rel_tr5(pre, post) == Y::State::tr_have_gen(pre, post),
// TODO                 rel_tr5_strong(pre, post) == Y::State::tr_have_gen_strong(pre, post),
// TODO             ]);
// TODO         }
// TODO
// TODO         proof fn test_inst1() {
// TODO             let tracked (Tracked(inst), Tracked(token_f)) = Y::Instance::initialize();
// TODO             assert(Set::empty().insert(spec_literal_int("19")).contains(spec_literal_int("19")));
// TODO             assert(token_f.dom().contains(spec_literal_int("19")));
// TODO             assert(equal(token_f.index(spec_literal_int("19")).view(), Y::token![
// TODO                 inst => b => spec_literal_int("19")
// TODO             ]));
// TODO
// TODO             let tracked token1 = inst.tr_add();
// TODO             assert(equal(token1.view().instance, inst));
// TODO             assert(token1.view().key == spec_literal_int("5"));
// TODO             inst.tr_have(&token1);
// TODO
// TODO             let token1_clone = token1.clone();
// TODO             assert(equal(token1_clone, token1));
// TODO
// TODO             let tracked token_set = inst.tr_add_gen();
// TODO             assert(Set::empty().insert(spec_literal_int("6")).contains(spec_literal_int("6")));
// TODO             assert(token_set.dom().contains(spec_literal_int("6")));
// TODO             assert(equal(token_set.index(spec_literal_int("6")).view(), Y::token![
// TODO                 inst => b => spec_literal_int("6")
// TODO             ]));
// TODO             inst.tr_have_gen(&token_set);
// TODO         }
// TODO
// TODO         }
// TODO     } => Ok(())
// TODO }
// TODO
// TODO test_verify_one_file! {
// TODO     #[test] tokenized_with_conditional IMPORTS.to_string() + verus_code_str! {
// TODO
// TODO         tokenized_state_machine!{ Y {
// TODO             fields {
// TODO                 #[sharding(variable)]
// TODO                 pub x: int,
// TODO
// TODO                 #[sharding(variable)]
// TODO                 pub y: int,
// TODO             }
// TODO
// TODO             init!{
// TODO                 initialize(x: int, y: int) {
// TODO                     init x = x;
// TODO                     init y = y;
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 upd() {
// TODO                     if pre.x == 0 {
// TODO                         update y = 1;
// TODO                     } else {
// TODO                         update y = 2;
// TODO                     }
// TODO                 }
// TODO             }
// TODO
// TODO             transition!{
// TODO                 req() {
// TODO                     if pre.x == 0 {
// TODO                         require(pre.y == 1);
// TODO                         update y = 20;
// TODO                     } else {
// TODO                         require(pre.y == 2);
// TODO                         update y = 25;
// TODO                     }
// TODO                 }
// TODO             }
// TODO         }}
// TODO
// TODO         verus!{
// TODO         proof fn test1() {
// TODO             let tracked (Tracked(inst), Tracked(x), Tracked(mut y)) = Y::Instance::initialize(spec_literal_int("0"), spec_literal_int("0"));
// TODO             inst.upd(&x, &mut y);
// TODO             assert(y.view().value == spec_literal_int("1"));
// TODO         }
// TODO
// TODO         proof fn test2() {
// TODO             let tracked (Tracked(inst), Tracked(x), Tracked(mut y)) = Y::Instance::initialize(spec_literal_int("12"), spec_literal_int("0"));
// TODO             inst.upd(&x, &mut y);
// TODO             assert(y.view().value == spec_literal_int("2"));
// TODO         }
// TODO
// TODO         proof fn test3() {
// TODO             let tracked (Tracked(inst), Tracked(x), Tracked(mut y)) = Y::Instance::initialize(spec_literal_int("0"), spec_literal_int("2"));
// TODO             inst.req(&x, &mut y); // FAILS
// TODO         }
// TODO
// TODO         proof fn test4() {
// TODO             let tracked (Tracked(inst), Tracked(x), Tracked(mut y)) = Y::Instance::initialize(spec_literal_int("1"), spec_literal_int("1"));
// TODO             inst.req(&x, &mut y); // FAILS
// TODO         }
// TODO         }
// TODO
// TODO     } => Err(e) => assert_fails(e, 2)
// TODO }
