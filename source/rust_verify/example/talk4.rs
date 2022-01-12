#[allow(unused_imports)] use builtin::*;
#[allow(unused_imports)] use builtin_macros::*;
mod pervasive; #[allow(unused_imports)] use pervasive::*;

#[allow(unused_imports)]
use cell::*;

#[exec]
fn increment(counter: &PCell<u64>, #[proof] permission: &mut Permission<u64>) {
    let cur = counter.borrow(permission);
    counter.put(cur + 1, permission); 
}

#[exec]
#[verifier(external_body)]
fn transfer(#[proof] permission: Permission<u64>) { unimplemented!() }

#[exec]
fn start_thread(counter: &PCell<u64>, #[proof] mut permission: Permission<u64>) {
    transfer(permission);
    increment(counter, &mut permission);
}

fn main() { }
