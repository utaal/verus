mod pervasive;
#[allow(unused_imports)]
use { builtin_macros::*, builtin::*, pervasive::*, option::*, seq::*, vec::*, };

#[spec]
fn divides(factor: nat, candidate: nat) -> bool
{
    candidate % factor == 0
}

#[spec]
fn is_prime(candidate: nat) -> bool
{
       true
    && 1 < candidate
    && forall(|factor: nat| 1 < factor && factor < candidate >>= !divides(factor, candidate))
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

fn test_prime(candidate: u64) -> bool
{
    requires(1 < candidate);
    ensures(|result: bool| result == is_prime(candidate));
    
    let mut factor:u64 = 2;
    while (factor < candidate)
    {
        invariant(forall(|smallerfactor:nat| 1 < smallerfactor && smallerfactor < factor >>= !divides(smallerfactor, candidate)));
        if candidate % factor == 0 {
            assert(divides(factor, candidate));
            assume(!is_prime(candidate));   // TODO(chris): can't prove the !forall. (Dafny doesn't need this line, either.)
            return false;
        }
        factor = factor + 1;
    }
    true
}

fn main()
{
}
