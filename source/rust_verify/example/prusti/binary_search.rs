mod pervasive;
use pervasive::prelude::*;
use pervasive::vec::*;

verus! {

pub open spec fn sorted(s: &Vec<i32>) -> bool {
    forall|i: int, j: int| 0 <= i < j < s.len() ==> s[i] <= s[j]
}

pub fn binary_search(s: &Vec<i32>, n: i32) -> (result: Option<usize>)
    requires sorted(s),
    ensures match result {
        Some(index) => index < s.len() && s[index as int] == n,
        None => forall|i: int| 0 <= i < s.len() ==> s[i] != n,
    }
{
    let mut base = 0;
    let mut size = s.len();

    while size > 0
        invariant
            sorted(s),
            base as int + size <= s.len(),
            forall|k: int| 0 <= k < base ==> s[k] < n,
            forall|i: int| base + size <= i < s.len() ==> s[i] != n,
    {
        let half = size / 2;
        let mid = base + half;

        size = size - half;
        if *s.index(mid) > n {
            // continue
        } else if *s.index(mid) < n {
            base = mid;
        } else {
            return Some(mid);
        }
    }

    None
}

fn main() {}

}
