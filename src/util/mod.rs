use core::fmt::{self, Write};

pub trait ConstDefault {
	const DEFAULT: Self;
}

pub fn _print(args: fmt::Arguments) {
	let _ = ::arm::semihosting::Printer.write_fmt(args);
}
