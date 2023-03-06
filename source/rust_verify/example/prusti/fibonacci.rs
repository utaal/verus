//! An adaptation of the example from
//! https://rosettacode.org/wiki/Fibonacci_sequence#Rust
//!
//!
//! Omitted:
//!
//! +   Analytical version:
//!
//!     +   Uses closures.
//!     +   Uses floating point numbers.
//!
//! Changes:
//!
//! +   Replaced ``println!`` with calling trusted functions.
//! +   Unified function types.
//! +   Renamed functions.
//! +   Added ghost counters to prove that all versions generate the same sequence.
//! +   Rewrote loops into supported shape (while bool with no break, continue, or return).
//! +   Rewrote closure into a match statement.
//! +   Replaced Iterator::next function with a function next.
//! +   Wrapped built-in types and functions.
//!
//! Verified properties:
//!
//! +   Absence of panics.
//! +   The verified three implementations print only Fibonacci numbers.

mod pervasive;
use pervasive::prelude::*;

verus! {

pub open spec fn fib(i: nat) -> nat
    decreases i
{
    if i == 0 { 0 } else if i == 1 { 1 }
    else { fib((i - 1) as nat) + fib((i - 2) as nat) }
}

#[verifier(external_body)]
fn swap(a: &mut usize, b: &mut usize)
    ensures b == old(a), a == old(b)
{
    std::mem::swap(a, b);
}

#[verifier(external_body)]
fn checked_add(a: usize, b: usize) -> (result: Option<usize>)
    ensures result.is_Some() ==> result.get_Some_0() == a + b
{
    match a.checked_add(b) {
        std::option::Option::Some(n) => Option::Some(n),
        std::option::Option::None => Option::None,
    }
}

#[verifier(external_body)]
pub struct FibPrinter {}

impl FibPrinter {
    pub closed spec fn last_printed(self) -> nat;

    #[verifier(external_body)]
    pub exec fn new() -> (result: Self)
        ensures result.last_printed() == 0
    {
        Self {}
    }

    #[verifier(external_body)]
    pub exec fn print(&mut self, c: Ghost<nat>, f: usize)
        requires
            c == old(self).last_printed() + 1,
            f == fib(c@),
        ensures self.last_printed() == c
    {
        println!("{}", f);
    }
}

// Iterative

fn iterative_fibonacci(fibo_printer: &mut FibPrinter)
    requires old(fibo_printer).last_printed() == 0
    // TODO ensures fib(fibo_printer.last_printed()) > usize::MAX
{
    let mut prev = 0;
    let mut curr = 1;

    #[allow(unused_mut)] // TODO spurious warning
    let mut ghost_counter: Ghost<nat> = ghost(1);

    loop
        invariant
            ghost_counter@ >= 1,
            fib(ghost_counter@) == curr,
            fib((ghost_counter@ - 1) as nat) == prev,
            fibo_printer.last_printed() == ghost_counter@ - 1,
    {
        fibo_printer.print(ghost_counter, curr);
        if let Some(n) = checked_add(curr, prev) {
            prev = curr;
            curr = n;
            proof {
                ghost_counter@ = ghost_counter@ + 1;
            }
        } else {
            break;
        }
    }
}

// Recursive

fn recursive_fibonacci(
    fibo_printer: &mut FibPrinter, ghost_counter: Ghost<nat>, prev: usize, curr: usize)
    requires
        old(fibo_printer).last_printed() == ghost_counter@ - 1,
        ghost_counter@ >= 1,
        fib((ghost_counter@ - 1) as nat) == prev,
        fib(ghost_counter@) == curr,
{
    fibo_printer.print(ghost_counter, curr);
    if let Option::Some(n) = checked_add(curr, prev) {
        recursive_fibonacci(fibo_printer, ghost(ghost_counter@ + 1), curr, n);
    }
}

// Using an Iterator

pub struct Fib {
    prev: usize,
    curr: usize,
    ghost_counter: Ghost<nat>,
}

impl Fib {
    // #[ensures(result.valid())]
    // #[ensures(result.counter() == 1)]
    pub fn new() -> (result: Self)
        ensures
            result.valid(),
            result.counter() == 1,
    {
        Fib { prev: 0, curr: 1, ghost_counter: ghost(1) }
    }

    pub closed spec fn counter(&self) -> nat {
        self.ghost_counter@
    }

    pub closed spec fn valid(&self) -> bool {
        &&& self.ghost_counter@ >= 1
        &&& self.prev == fib((self.ghost_counter@ - 1) as nat)
        &&& self.curr == fib(self.ghost_counter@)
    }

    fn next(&mut self) -> (result: Option<usize>)
        requires old(self).valid()
        ensures
            result.is_Some() ==> self.counter() == old(self).counter() + 1,
            result.is_Some() ==> self.valid(),
            result.is_Some() ==> fib(self.counter()) == result.get_Some_0(),
    {
        swap(&mut self.curr, &mut self.prev);
        if let Some(n) = checked_add(self.curr, self.prev) {
            self.curr = n;
            proof {
                self.ghost_counter@ = self.ghost_counter@ + 1;
            }
            Some(n)
        }
        else {
            None
        }
    }
}

fn fibonacci_iterator(fibo_printer: &mut FibPrinter)
    requires old(fibo_printer).last_printed() == 0
{
    let mut iter = Fib::new();
    fibo_printer.print(ghost(1), 1);
    loop
        invariant
            iter.valid(),
            fibo_printer.last_printed() == iter.counter(),
    {
        match iter.next() {
            Some(n) => {
                let i: Ghost<nat> = ghost(iter.counter());
                fibo_printer.print(i, n);
            }
            None => {
                break;
            }
        }
    }
}

fn main() {
    let mut fibo_printer = FibPrinter::new();
    iterative_fibonacci(&mut fibo_printer);
    let mut fibo_printer = FibPrinter::new();
    recursive_fibonacci(&mut fibo_printer, ghost(1), 0, 1);
    let mut fibo_printer = FibPrinter::new();
    fibonacci_iterator(&mut fibo_printer);
}

}
