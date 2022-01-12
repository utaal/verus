#[allow(unused_imports)] use builtin::*;
#[allow(unused_imports)] use builtin_macros::*;
mod pervasive; #[allow(unused_imports)] use pervasive::*;

fn main() {
    let mut x;
    x = 3;
    assert(x == 3);
}
