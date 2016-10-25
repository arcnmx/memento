#[macro_export]
macro_rules! println {
	($fmt:expr) => (print!(concat!($fmt, "\n")));
	($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

#[macro_export]
macro_rules! print {
	($($arg:tt)*) => ($crate::util::_print(format_args!($($arg)*)));
}
