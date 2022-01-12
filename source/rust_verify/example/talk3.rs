#[allow(unused_imports)] use builtin::*;
#[allow(unused_imports)] use builtin_macros::*;
mod pervasive; #[allow(unused_imports)] use pervasive::*;

#[allow(unused_imports)]
use seq::*;
#[allow(unused_imports)]
use vec::*;

#[spec]
fn fibo(n: nat) -> nat {
    decreases(n);
    if n == 0 { 0 }
    else if n == 1 { 1 }
    else { fibo(n - 2) + fibo(n - 1) }
}

#[proof]
fn lemma_fibo_is_monotonic(i:nat, j:nat) {
    requires(i<=j);
    ensures(fibo(i) <= fibo(j));
    decreases(j-i);

    if i<2 && j<2 {
    } else if i==j {
    } else if i==j-1 {
        reveal_with_fuel(fibo, 2); lemma_fibo_is_monotonic(i, j-1);
    } else {
        lemma_fibo_is_monotonic(i, j-1); lemma_fibo_is_monotonic(i, j-2);
    }
}

#[spec]
fn fibo_in_bound_u64(n: nat) -> bool { fibo(n) <= 0xffff_ffff_ffff_ffff }

#[exec]
fn fibo_impl(n: u64) -> u64 {
    requires(fibo_in_bound_u64(n));
    ensures(|result: u64| result == fibo(n));
    if n == 0 { return 0; } 
    let mut prev: u64 = 0; let mut cur: u64 = 1; let mut i: u64 = 1;
    while i < n {
        invariant([i > 0, i <= n,
            fibo_in_bound_u64(n as nat) && fibo_in_bound_u64(i as nat),
            cur == fibo(i),
            prev == fibo(i as nat - 1),
        ]);
        let new_cur = cur + prev;
        prev = cur; cur = new_cur; i = i + 1;
        lemma_fibo_is_monotonic(i, n);
    }
    cur
}

fn main() {}

// TODO: consider ADT
// TODO: linearity
// TODO: consider erasure
