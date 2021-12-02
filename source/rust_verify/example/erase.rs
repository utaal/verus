extern crate builtin;
#[allow(unused_imports)]
use builtin::*;
mod pervasive;
#[allow(unused_imports)]
use pervasive::*;

fn if_spec_cond(#[spec] i: int) {
    let mut a: u64 = 2;
    if i == 3 {
        assert(true);
    }
    a = a + 1;
    assert(a == 3);
}
