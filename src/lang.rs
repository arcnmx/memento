use core::fmt;

#[lang = "panic_fmt"]
extern fn rust_begin_unwind(msg: fmt::Arguments, file: &'static str, line: u32) -> ! {
	use core::intrinsics::abort;

	unsafe {
		println!("panic at {}:{}: {}", file, line, msg);
		abort();
	}
}

#[lang = "eh_personality"]
unsafe extern fn rust_eh_personality() { unimplemented!() }

#[lang = "eh_unwind_resume"]
unsafe extern fn rust_eh_unwind_resume() { unimplemented!() }
