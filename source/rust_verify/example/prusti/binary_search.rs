mod pervasive;
use pervasive::prelude::*;

verus! {

pub enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

impl<T> List<T> {
    pub open spec fn spec_len(self) -> nat
        decreases self
    {
        match self {
            List::Cons(_, ls) => 1 + ls.spec_len(),
            List::Nil => 0,
        }
    }

    pub open spec fn spec_index(self, ix: int) -> T
        decreases self
    {
        match self {
            List::Cons(t, ls) => {
                if ix == 0 {
                    t
                } else {
                    ls.spec_index(ix - 1)
                }
            }
            List::Nil => arbitrary(),
        }
    }

    fn index(&self, ix: usize) -> (result: &T)
        requires ix < self.spec_len()
        ensures *result == self.spec_index(ix as int)
    {
        let orig_ix: Ghost<usize> = ghost(ix);
        let mut ix = ix;
        let mut l = self;

        loop
            invariant
                ix < l.spec_len(),
                self.spec_index(orig_ix@ as int) == l.spec_index(ix as int),
            ensures self.spec_index(orig_ix@ as int) == l.spec_index(0)
        {
            if let List::Cons(t, ls) = l {
                if ix > 0 {
                    l = &*ls;
                    ix = ix - 1;
                } else {
                    assume(false); // TODO
                    return t;
                }
            } else {
                unreached()
            }
        }
    }

    fn len(&self) -> (result: usize)
        requires self.spec_len() < usize::MAX
        ensures result == self.spec_len()
    {
        let mut len: usize = 0;
        let mut l = self;
        loop
            invariant self.spec_len() < usize::MAX
            invariant_ensures len + l.spec_len() == self.spec_len()
            ensures l.spec_len() == 0
        {
            if let List::Cons(_, ls) = l {
                len = len + 1;
                l = ls;
            } else {
                break;
            }
        }
        len
    }
}

impl List<u32> {
    pub open spec fn is_sorted(&self) -> bool {
        forall|x1:int, x2:int| 0 <= x1 <= x2 < self.spec_len() ==>
            self.spec_index(x1) <= self.spec_index(x2)
    }
}

pub fn binary_search(arr: &List<u32>, elem: u32) -> (result: Result<usize, usize>)
    requires
        arr.spec_len() < usize::MAX,
        arr.is_sorted(),
    ensures
        forall|x: usize| result == Result::<usize, usize>::Ok(x) ==> #[trigger] arr.spec_index(x as int) == elem,
        forall|x: usize| result == Result::<usize, usize>::Err(x) ==>
            forall|i: usize| 0 <= i < x ==> arr.spec_index(i as int) <= elem,
        forall|x:usize| result == Result::<usize, usize>::Err(x) ==>
            forall|i:usize| x < i < arr.spec_len() ==> elem < arr.spec_index(i as int)
{
    if arr.len() == 0 {
        return Err(0);
    }
    let mut size = arr.len();
    let mut base = 0;

    while size > 1
        invariant
            0 < size && size + base <= arr.spec_len() < usize::MAX,
            arr.is_sorted(),
            forall|i: nat| i < base ==> arr.spec_index(i as int) <= elem,
            forall|i: nat| base as int + size < i < arr.spec_len() ==> elem < arr.spec_index(i as int),
    {
        let half = size / 2;
        let mid = base + half;

        base = if *arr.index(mid) > elem { base } else { mid };
        size = size - half;
    }

    let cmp = *arr.index(base);
    if cmp == elem {
        Ok(base)
    } else if cmp < elem {
        Err(base + 1)
    } else {
        Err(base)
    }
}

fn main() {}

}
