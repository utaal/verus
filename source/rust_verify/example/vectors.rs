#[allow(unused_imports)]
use builtin::*;
#[allow(unused_imports)]
use builtin_macros::*;
mod pervasive;
#[allow(unused_imports)]
use crate::pervasive::{*, seq::*, modes::*};

#[verifier(external_body)]
pub struct Vec<#[verifier(strictly_positive)] A> {
    pub vec: std::vec::Vec<A>,
}

impl<A> Vec<A> {
    #[spec] #[verifier(external_body)] pub fn view(&self) -> Seq<A> { unimplemented!() }

    #[verifier(external_body)]
    pub fn new() -> Self {
        ensures(|v: Self| equal(v.view(), Seq::empty()));
        Vec { vec: std::vec::Vec::new() }
    }
    
    #[verifier(external_body)]
    pub fn push(&mut self, value: A) {
        ensures(equal(self.view(), old(self).view().push(value)));
        self.vec.push(value);
    }

    #[verifier(external_body)]
    pub fn pop(&mut self) -> A {
        requires(old(self).len() > 0);
        ensures(|value: A| [equal(value, old(self).view().index((old(self).view().len() - 1) as int)),
            equal(self.view(), old(self).view().subrange(0, (old(self).view().len() - 1) as int))]);
        unsafe {
            self.vec.pop().unwrap_unchecked()  // Safe to unwrap given the precondition above
        }
    }

    #[verifier(external_body)]
    #[verifier(autoview)]
    pub fn index(&self, i: usize) -> &A {
        requires(i < self.len());
        ensures(|r: &A| equal(*r, self.view().index(i as int)));
        &self.vec[i]
    }

    #[verifier(external_body)]
    pub fn set(&mut self, i: usize, a: A) {
        requires(i < old(self).len());
        ensures(equal(self.view(), old(self).view().update(i as int, a)));
        self.vec[i] = a;
    }

    #[verifier(external_body)]
    #[verifier(autoview)]
    pub fn len(&self) -> usize {
        ensures(|l: usize| l == self.len());
        self.vec.len()
    }
}


#[exec] fn binary_search(v: &Vec<u64>, k: u64) -> usize {
    requires([
        forall(|i:int, j:int| imply(0 <= i && i <= j && j < v.len(), v.index(i) <= v.index(j))),
        exists(|i:int| 0 <= i && i < v.len() && k == v.index(i))]);
    ensures(|r: usize| [
        r < v.len(),
        k == v.index(r as int)]);
    let mut i1: usize = 0;
    let mut i2: usize = v.len() - 1;
    while i1 != i2 {
        invariant([
            i2 < v.len(),
            exists(|i:int| i1 <= i && i <= i2 && k == v.index(i)),
            forall(|i:int, j:int| imply(0 <= i && i <= j && j < v.len(), v.index(i) <= v.index(j)))]);

        #[spec] let d = i2 - i1;

        let ix = i1 + (i2 - i1) / 2;
        if *v.index(ix) < k {
            i1 = ix + 1;
        } else {
            i2 = ix;
        }

        assert(i2 - i1 < d);
    }
    i1
}

fn reverse(v: &mut Vec<u64>) {
    ensures([
        v.len() == old(v).len(),
        forall(|i: int| imply(0 <= i && i < old(v).len(), v.index(i) == old(v).index(old(v).len() - i - 1)))]);

    let length = v.len();
    #[spec] let v1: Seq<u64> = v.view();
    let mut n: usize = 0;
    while n < length / 2 {
        invariant([
            length == v.len(),
            forall(|i: int| imply(0 <= i && i < n, v.index(i) == v1.index(length - i - 1))),
            forall(|i: int| imply(0 <= i && i < n, v1.index(i) == v.index(length - i - 1))),
            forall(|i: int| imply(n <= i && i + n < length, #[trigger] v.index(i) == v1.index(i))),
        ]);

        let x = *v.index(n);
        let y = *v.index(length - 1 - n);
        v.set(n, y);
        v.set(length - 1 - n, x);

        n = n + 1;
    }
}

verus! {

// fn pusher() -> Vec<u64> {
//     let mut v = Vec::new();
//     v.push(0);
//     v.push(1);
//     v.push(2);
//     v.push(3);
//     v.push(4);
//     let goal: Ghost<Seq<u64>> = ghost(Seq::new(5, |i: int| i as u64));
//     assert(v@.ext_equal(goal@));
//     assert(v[2] == 2);
// 
//     v.pop();
//     v.push(4);
//     assert(v@.ext_equal(goal@));
// 
//     v
// }
// 
// spec fn uninterp_fn(x: u64) -> bool;
// 
// fn pop_test(t: Vec<u64>)
//     requires
//         t.len() > 0,
//         forall|i: int| #![auto] 0 <= i < t.len() ==> uninterp_fn(t[i]),
// {
//     let mut t = t;
//     let x = t.pop();
// 
//     assert(uninterp_fn(x));
//     assert(forall|i: int| #![auto] 0 <= i < t.len() ==> uninterp_fn(t[i]));
// }
// 
// fn pust_test(t: Vec<u64>, y: u64)
//     requires
//         forall|i: int| #![auto] 0 <= i < t.len() ==> uninterp_fn(t[i]),
//         uninterp_fn(y),
// {
//     let mut t = t;
//     t.push(y);
// 
//     assert(
//         forall|i: int| #![auto] 0 <= i < t.len() ==> uninterp_fn(t[i])
//     );
// }

#[verifier(external)]
fn main() {
//     let mut v = Vec{vec: vec![0, 10, 20, 30, 40, 50, 60, 70, 80, 90]};
//     println!("{}", binary_search(&v, 70));
//     println!();
//     reverse(&mut v);
//     for x in v.vec {
//         println!("{}", x);
//     }
// 
//     println!("Pushed 5 values:");
//     for x in pusher().vec {
//         println!("{}", x);
//     }
}

} // verus!


