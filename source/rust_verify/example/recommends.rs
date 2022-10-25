#[allow(unused_imports)]
use builtin::*;
use builtin_macros::*;
mod pervasive;
#[allow(unused_imports)]
use crate::pervasive::{*, seq::*, seq_lib::*};


#[spec] fn max(x: int, y: int) -> int {
    if x > y { x } else { y }
}

#[spec] fn seq_max(s: Seq<int>) -> int {
    recommends(s.len() > 0);
    decreases(s.len());
    let m = s.index((s.len() - 1) as int);
    if s.len() <= 1 {
        m
    } else {
        max(m, seq_max(s.drop_last()))
    }
}

#[proof] fn lemma1(s1: Seq<int>, s2: Seq<int>) {
    requires(s1.len() == 0 && s2.len == 0);
    ensures(seq_max(s1) == seq_max(s2));
}

#[proof] fn lemma2(s: Seq<int>) {
    // requires(seq_max(s) >= 0) // without this, the assertion fails and there's a recommends note
    ensures(seq_max(s) >= 0);
}

verus! {

fn main() {
    proof {
        let s = seq![10, 20, 30, 25];
        reveal_with_fuel(seq_max, 4);
        assert(seq_max(s) == 30);
    }
}

// Usage of `spec_affirm`
spec fn some_predicate(a: nat) -> bool
    recommends a < 100
{
    if (a >= 50) {
        let _ = spec_affirm(50 <= a && a < 100);
        a >= 75
    } else {
        let _ = spec_affirm(a < 40); // spec(checked) would raise a recommends note here
        a < 25
    }
}

} // verus!
