mod pervasive;
#[allow(unused_imports)]
use { builtin_macros::*, builtin::*, pervasive::*, option::*, seq::*, vec::*, };

#[spec]
fn is_sorted(seq: Seq<u64>) -> bool {
    forall(|i: nat, j: nat| i < j && j < seq.len() >>= seq.index(i) <= seq.index(j))
}

fn is_vec_sorted(vec: Vec<u64>) -> bool {
    ensures(|ret: bool| ret == is_sorted(vec.view()));

    if vec.len() < 2 {
        return true;
    }

    let mut idx: usize = 0;
    while idx < vec.len() - 1
    {
        invariant([
            idx < vec.len(),
            forall(|i: nat, j: nat| i < j && j < idx as nat + 1 >>= vec.index(i) <= vec.index(j)),
        ]);
        if vec.index(idx) > vec.index(idx + 1) { // vec[idx]
            return false;
        }
        idx = idx + 1;
    }
    true
}

fn main() {}
