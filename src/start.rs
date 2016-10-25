use core::ptr;

pub unsafe fn init_bss() {
	extern {
		static mut __bss_start: u8;
		static mut __bss_end: u8;
	}

	let bss_start = &mut __bss_start as *mut u8;
	let bss_end = &mut __bss_end as *mut u8;
	let bss_length = bss_end as usize - bss_start as usize;
	ptr::write_bytes(bss_start, 0, bss_length);
}

pub unsafe fn init_data() {
	extern {
		static __data_rom: u8;
		static mut __data_start: u8;
		static mut __data_end: u8;
	}

	let data_rom = &__data_rom as *const u8;
	let data_start = &mut __data_start as *mut u8;
	let data_end = &mut __data_end as *mut u8;
	let data_length = data_end as usize - data_start as usize;
	ptr::copy_nonoverlapping(data_rom, data_start, data_length);
}

#[no_mangle]
#[inline(always)]
pub unsafe extern fn __start() -> ! {
	use core::ptr;

	extern {
		fn main(argc: isize, argv: *const *const u8) -> isize;
	}

	main(0, ptr::null());

	panic!("execution returned from main");
}
