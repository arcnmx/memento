#![feature(const_fn)]

extern crate memento;

pub use memento::arch::cortex_m0::isr::*;
pub use memento::arch::*;

static mut isr: ExceptionVectors = ExceptionVectors {
	initial_sp: &__STACK_START,
	.. ExceptionVectors::DEFAULT
};

fn main() {

}
