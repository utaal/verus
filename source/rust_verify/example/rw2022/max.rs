mod pervasive;
#[allow(unused_imports)]
use { builtin_macros::*, builtin::*, pervasive::*, option::*, seq::*, vec::*, };


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

fn main() {
    let x = 3;
    let y = 4;
    let m = max(x, y);
    assert(m == x || m == y);
}
