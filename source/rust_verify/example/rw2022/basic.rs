mod pervasive;
#[allow(unused_imports)]
use { builtin_macros::*, builtin::*, pervasive::*, option::*, seq::*, vec::*, };

fn main() {
    let x = 3;
    let y = 4;
    assert(x != y);
}

#[derive(Eq, PartialEq, Structural)]
struct Train {
    cars: u64,
}

fn main2() {
    let t = Train { cars: 10 };
    let q = Train { cars: 10 };
    assert(t == q);
}

#[spec]
fn mul(a: u64, b: u64)  -> u64 {
    a * b
}

// ;; Function-Decl crate::mul
// (declare-fun mul.? (Poly Poly) Int)
//
// ;; Function-Axioms crate::mul
// (assert
//  (fuel_bool_default fuel%mul.)
// )
// (assert
//  (=>
//   (fuel_bool fuel%mul.)
//   (forall ((a@ Poly) (b@ Poly)) (!
//     (= (mul.? a@ b@) (uClip 64 (* (%I a@) (%I b@))))
//     :pattern ((mul.? a@ b@))
// ))))
// (assert
//  (forall ((a@ Poly) (b@ Poly)) (!
//    (=>
//     (and
//      (has_type a@ (UINT 64))
//      (has_type b@ (UINT 64))
//     )
//     (uInv 64 (mul.? a@ b@))
//    )
//    :pattern ((mul.? a@ b@))
// )))

#[spec]
fn divides(v: u64, d: u64) -> bool {
    exists(|k: u64| mul(d, k) == v)
}

#[verifier(external)]
fn gcd_external(a: u64, b: u64) -> u64 {
    let mut i = a;
    while i >= 1 {
        if a % i == 0 && b % i == 0 {
            break;
        }
        i -= 1;
    }
    i
}

#[verifier(external_body)]
fn gcd(a: u64, b: u64) -> u64 {
    requires([a >= 0, b >= 0]);
    ensures(|result: u64| [divides(a, result), divides(b, result)]);

    gcd_external(a, b)
}

fn main3() {
    let x = 42;
    let y = 182;

    let z = gcd(x, y);

    assert(divides(x, z));
    assert(divides(y, z));
    // TOOD assert(x % z == 0);
}
