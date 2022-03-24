mod pervasive;
#[allow(unused_imports)]
use { builtin_macros::*, builtin::*, pervasive::*, option::*, seq::*, vec::*, };

#[is_variant]
enum Tree {
    Nil,
    Node { value: i64, left: Box<Tree>, right: Box<Tree> },
}

impl Tree {
    #[spec] fn view(&self) -> Seq<int> {
        decreases(self);
        match *self {
            Tree::Nil => seq![],
            Tree::Node { value, left, right } => left.view().add(seq![value as int]).add(right.view()),
        }
    }

    #[spec] fn is_sorted(&self) -> bool {
        decreases(self);
        match *self {
            Tree::Nil => true,
            Tree::Node { value, left, right } => true
                && sequences_ordered_at_interface(left.view(), seq![value])
                && sequences_ordered_at_interface(seq![value], right.view())
                && left.is_sorted()
                && right.is_sorted()
        }
    }
}

#[spec]
fn sequences_ordered_at_interface(seq1: Seq<int>, seq2: Seq<int>) -> bool {
    if seq1.len() == 0 || seq2.len() == 0 {
        true
    } else {
        seq1.last() <= seq2.index(0)
    }
}

#[spec] fn sequence_is_sorted(s: Seq<int>) -> bool {
    forall(|i: nat, j: nat| i < j && j < s.len() >>= s.index(i) <= s.index(j))
}

#[proof] fn sorted_tree_means_sorted_sequence(tree: Tree) {
    decreases(tree);
    requires(tree.is_sorted());
    ensures(sequence_is_sorted(tree.view()));

    if let Tree::Node { left, right, value: _ } = tree {
        sorted_tree_means_sorted_sequence(*left);
        sorted_tree_means_sorted_sequence(*right);
    }
}

#[is_variant]
enum TreeSortedness {
    Unsorted,
    Empty,
    Bounded(i64, i64),
}

fn find_in_binary_tree(tree: &Tree, needle: i64) -> bool {
    decreases(tree);
    requires(tree.is_sorted());
    ensures(|ret: bool| ret == tree.view().contains(needle));
    
    match tree {
        Tree::Nil => false,
        Tree::Node { left, value, right } => {
            if needle == *value {
                assert(tree.view().index(left.view().len()) == needle); // trigger
                true
            } else if needle < *value {
                let ret = find_in_binary_tree(left, needle);
                if ret {
                    #[spec] let idx = choose(|idx: nat| idx < left.view().len() && left.view().index(idx) == needle);
                    assert(tree.view().index(idx) == needle);   // trigger
                } else {
                    sorted_tree_means_sorted_sequence(**right);
                }
                ret
            } else {
                let ret = find_in_binary_tree(right, needle);
                if ret {
                    #[spec] let idx = choose(|idx: nat| idx < right.view().len() && right.view().index(idx) == needle);
                    assert(tree.view().index(left.view().len() as int + 1 + idx) == needle);   // trigger
                } else {
                    sorted_tree_means_sorted_sequence(**left);
                }
                ret
            }
        }
    }
}

fn main() {}
