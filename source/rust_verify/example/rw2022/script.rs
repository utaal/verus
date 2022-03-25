fn main() {}

// ## 11 -- 10-program.rs

mod pervasive;
#[allow(unused_imports)] use { builtin_macros::*, builtin::*, pervasive::*, option::*, seq::*, vec::*, };

fn max(a: u64, b: u64) -> u64 {
    ensures(|ret: u64| [
        ret == a || ret == b,
        ret >= a && ret >= b,
    ]);

    if a >= b {
        a
    } else {
        b
    }
}

fn max_test1() {
    let x = 3;
    let y = 4;
    let ret = max(x, y);
    assert(ret == x || ret == y);
    assert(ret >= x || ret >= y);
}

fn max_test2() {
    let x = 3;
    let y = 4;
    let ret = max(x, y);
    assert(ret == 4);
}


// ## 13 -- 13-program.rs

fn main_1() {
    let x = 3;
    let y = 4;
    assert(x != y);
}

// ## 13 -- 13-program.rs.smt sat

// ## 14 -- 14-prime.rs

#[spec]
fn divides(factor: nat, candidate: nat) -> bool {
    candidate % factor == 0
}

#[spec]
fn is_prime(candidate: nat) -> bool {
       1 < candidate
    && forall(|factor: nat| 1 < factor && factor < candidate >>= !divides(factor, candidate))
}

/*
fn test_prime(candidate: u64) -> bool {
    requires(1 < candidate);
    ensures(|result: bool| result == is_prime(candidate));
    
    let mut factor: u64 = 2;
    while (factor < candidate) {
        if candidate % factor == 0 {
            return false;
        }
        factor = factor + 1;
    }
    true
}
*/

fn test_prime(candidate: u64) -> bool {
    requires(1 < candidate);
    ensures(|result: bool| result == is_prime(candidate));
    
    let mut factor: u64 = 2;
    while (factor < candidate) {
        invariant([
            1 < factor, factor <= candidate,
            forall(|smallerfactor:nat| 1 < smallerfactor && smallerfactor < factor
                   >>= !divides(smallerfactor, candidate))
        ]);
        if candidate % factor == 0 {
            assert(divides(factor, candidate));
            assert(!is_prime(candidate));
            return false;
        }
        factor = factor + 1;
    }
    true
}

#[proof]
fn assertions() {
    assert(divides(3, 6));
    assert(divides(12, 24));
    assert(is_prime(2));
    assert(is_prime(3));
    assert(!divides(4, 5));
    assert(is_prime(5));
}

// ## 15 -- 15-fibo.rs


