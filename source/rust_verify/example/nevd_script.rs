#![feature(fmt_internals)] #![allow(unused_imports)] #![allow(unused_macros)]
fn main() {} mod pervasive;
 use { builtin_macros::*, builtin::*, pervasive::*, option::*, seq::*, vec::*, cell::*, option::Option::*};

verus! {

// ## A -- A-program.rs

fn max(a: u64, b: u64) -> (ret: u64)
    ensures
        ret == a || ret == b,
        ret >= a && ret >= b,
{
    //-   if a >= b { b } else { a }
    /*+*/ if a >= b { a } else { b }
}

// ## B -- B-fibo.rs

spec fn fibo(n: nat) -> nat
    decreases n
{
    if n == 0 { 0 } else if n == 1 { 1 }
    else { fibo((n - 2) as nat) + fibo((n - 1) as nat) }
}

proof fn lemma_fibo_is_monotonic(i: nat, j: nat)
    requires i <= j,
    ensures fibo(i) <= fibo(j),
    decreases j - i
{
   if i < 2 && j < 2 {
   } else if i == j {
   } else if i == j - 1 {
       reveal_with_fuel(fibo, 2);
       lemma_fibo_is_monotonic(i, (j - 1) as nat);
   } else {
       lemma_fibo_is_monotonic(i, (j - 1) as nat);
       lemma_fibo_is_monotonic(i, (j - 2) as nat);
   }
}

spec fn fibo_fits_u64(n: nat) -> bool {
    fibo(n) <= 0xffff_ffff_ffff_ffff
}

exec fn fibo_impl(n: u64) -> (result: u64)
    requires fibo_fits_u64(n as nat),
    ensures result == fibo(n as nat),
{
    if n == 0 {
        return 0;
    }
    let mut prev: u64 = 0;
    let mut cur: u64 = 1;
    let mut i: u64 = 1;
    while i < n
        invariant
            0 < i <= n,
            fibo_fits_u64(n as nat),
            fibo_fits_u64(i as nat),
            cur == fibo(i as nat),
            prev == fibo((i - 1) as nat),
    {
        i = i + 1;
        proof { lemma_fibo_is_monotonic(i as nat, n as nat); }
        let new_cur = cur + prev;
        prev = cur;
        cur = new_cur;
    }
    cur
}

// ## C -- C-linearity.rs

//-  exec fn f(v: Vec<u64>) -> (Vec<u64>, Vec<u64>) {
//-      let v1 = v;
//-      let v2 = v;
//-      (v1, v2)
//-  }

/*+*/ exec fn f(v: Vec<u64>) {
/*+*/     let v1: Ghost<Vec<u64>> = ghost(v);
/*+*/     let v2: Ghost<Vec<u64>> = ghost(v);
/*+*/     assert(v1@.len() == v2@.len());
/*+*/ }

exec fn g(v1: &mut Vec<u64>, v2: &mut Vec<u64>)
    requires
        old(v1)@.len() == 2,
        old(v2)@.len() == 3,
    ensures
        v1@.len() == v2@.len()
{
    v1.push(42);
    v1.push(43);
    v2.push(52);
}

// ## D -- D-recommends-solvers.rs

spec fn divide(x: nat, y: nat) -> nat
    recommends y != 0
{
    x / y
}

proof fn div_is_smaller(x: nat, y: nat)
    requires
        y != 0
    ensures
        divide(x, y) <= x,
{
/*+*/    assert(y != 0 ==> x / y <= x) by(nonlinear_arith);
}

// ## E -- E-solvers.rs

proof fn bit_vector_demo(x: u64, y: u64) {
    assert(x & 0xff < 0x100) by(bit_vector);
    assert(x ^ x == 0) by(bit_vector);
    assert(x ^ y == y ^ x) by(bit_vector);
    assert(x & y <= x | y) by(bit_vector);
}

// F -- F-linear-proof.rs
#[verifier(external_body)]
fn release_perm(perm: Tracked<PermissionOpt<u64>>) { todo!() }

fn increment(
    counter: PCell<u64>,
    perm: &mut Tracked<PermissionOpt<u64>>,
)
    requires
        counter.id() === old(perm)@@.pcell,
        old(perm)@@.value.is_Some() &&
        old(perm)@@.value.get_Some_0() < 100,
    ensures
        perm@@.pcell === old(perm)@@.pcell,
        perm@@.value === Some((old(perm)@@.value.get_Some_0() + 1) as u64)
{
    let cur_i: u64 = *counter.borrow(perm);
    counter.replace(perm, cur_i + 1);
}

fn start_thread(counter: PCell<u64>, perm: Tracked<PermissionOpt<u64>>)
    requires
        counter.id() === perm@@.pcell, perm@@.value === None,
{
    let mut perm: Tracked<PermissionOpt<u64>> = perm;
    counter.put(&mut perm, 5);
    assert(perm@@.value === Some(5));

    // release_perm(perm);
    increment(counter, &mut perm);
    assert(perm@@.value === Some(6));
}

} // verus!
