use builtin_macros::*;
use builtin::*;
mod pervasive;
use pervasive::{*, option::Option, option::Option::*};

// TODO The verifier does not yet support the following Rust feature: method call to method not defined in this crate
// #[spec] fn max<V: SpecOrd>(a: V, b: V) -> V {
//     if a.spec_ge(b) { a } else { b }
// }
// 
// #[spec] fn min<V: SpecOrd>(a: V, b: V) -> V {
//     if a.spec_le(b) { a } else { b }
// }
// 
// pub enum Node<V> {
//     Full { left: Box<Node<V>>, right: Box<Node<V>>, value: V, },
//     Empty,
// }
// 
// impl<V: SpecOrd> Node<V> {
//     #[spec] pub fn height(self) -> nat {
//         decreases(self);
//         match self {
//             Node::Full { left, right, value: _ } => max(left.height(), right.height()),
//             Node::Empty => 0,
//         }
//     }
// 
//     #[spec] pub fn balanced(self) -> bool {
//         match self {
//             Node::Full { left, right, value: _ } =>
//                 left.height() <= 2 * right.height() && right.height() <= 2 * left.height(),
//             Node::Empty => true,
//         }
//     }
// 
//     #[spec] pub fn bounds(self) -> Option<(V, V)> {
//          match self {
//             Node::Full { left, right, value } => {
//                 let low = if let Some((llow, _)) = left.bounds() { min(llow, value) } else { value };
//                 let high = if let Some((_, rhigh)) = right.bounds() { max(rhigh, value) } else { value };
//                 Some((low, high))
//             },
//             Node::Empty => None,
//         }
//     }
// 
//     // #[spec] pub fn inv(self) -> bool {
//     //     self.balanced() && match self {
//     //         Node::Full { left, right, value } => {
//     //             if let Node::Full { value: l, .. } = left { l <= value } &&
//     //             if let Node::Full { value: r, .. } = right { value <= r }
//     //         },
//     //         Node::Empty => true,
//     //     }
//     // }
// }

#[spec] fn max_u64(a: u64, b: u64) -> u64 {
    if a.spec_ge(b) { a } else { b }
}

#[spec] fn min_u64(a: u64, b: u64) -> u64 {
    if a.spec_le(b) { a } else { b }
}

#[spec] fn max(a: nat, b: nat) -> nat {
    if a.spec_ge(b) { a } else { b }
}

#[spec] fn min(a: nat, b: nat) -> nat {
    if a.spec_le(b) { a } else { b }
}

pub enum Node {
    Full { left: Box<Node>, right: Box<Node>, value: u64, },
    Empty,
}

impl Node {
    #[spec] pub fn height(self) -> nat {
        decreases(self);
        match self {
            Node::Full { left, right, value: _ } => max(left.height(), right.height()) + 1,
            Node::Empty => 0,
        }
    }

    #[spec] pub fn balanced(self) -> bool {
        match self {
            Node::Full { left, right, value: _ } =>
                left.height() <= 2 * right.height() && right.height() <= 2 * left.height(),
            Node::Empty => true,
        }
    }

    #[spec] pub fn bounds(self) -> Option<(u64, u64)> {
        decreases(self);
        match self {
            Node::Full { left, right, value } => {
                let low = if let Some((llow, _)) = left.bounds() { min_u64(llow, value) } else { value };
                let high = if let Some((_, rhigh)) = right.bounds() { max_u64(rhigh, value) } else { value };
                Some((low, high))
            },
            Node::Empty => None,
        }
    }

    #[spec] pub fn inv(self) -> bool {
        decreases(self);
        self.balanced() && {
            match self {
                Node::Full { left, right, value } => {
                    left.inv() &&
                    right.inv() &&
                    if let Some((_, lhigh)) = left.bounds() { lhigh <= value } else { true } &&
                    if let Some((rlow, _)) = right.bounds() { value <= rlow } else { true }
                },
                Node::Empty => true,
            }
        }
    }
}
fn main() {}

