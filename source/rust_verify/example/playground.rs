use builtin_macros::*;
use builtin::*;
mod pervasive;
use pervasive::{*, option::Option, result::Result};

use pervasive::seq::*;
use crate::pervasive::vec::*;

#[spec] fn f(i: nat) -> nat
{
    recommends(i > 0);
    (i - 1) as nat
}

#[proof] fn test1() {
    assert(f(0) == f(0)); // succeeds
}

#[proof] fn test2() {
    assert(f(0) <= f(1)); // fails
}

fn main() {}

