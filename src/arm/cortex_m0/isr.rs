use libc::c_void;
use util::ConstDefault;
use arch::{IrqHandler, __STACK_START};

pub type VectorTable<I> = arm::VectorTable<ExceptionVectors, I>;

extern fn cortex_m0_isr() {
	unsafe {
		asm!("
			.thumb_func
			.global __default_isr_handler
			__default_isr_handler:
			bkpt
			b .
		"::::"volatile");
	}
}

#[repr(C)]
pub struct ExceptionVectors {
	pub initial_sp: *const c_void,
	pub reset: IrqHandler,
	pub nmi: IrqHandler,
	pub hard_fault: IrqHandler,
	pub reserved_4: [IrqHandler; 7],
	pub svcall: IrqHandler,
	pub reserved_12: [IrqHandler; 2],
	pub pendsv: IrqHandler,
	pub systick: IrqHandler,
}

impl ConstDefault for ExceptionVectors {
	const DEFAULT: ExceptionVectors = ExceptionVectors {
		initial_sp: &__STACK_START,
		reset: IrqHandler::DEFAULT,
		nmi: IrqHandler::DEFAULT,
		hard_fault: IrqHandler::DEFAULT,
		reserved_4: [IrqHandler::DEFAULT; 7],
		svcall: IrqHandler::DEFAULT,
		reserved_12: [IrqHandler::DEFAULT; 2],
		pendsv: IrqHandler::DEFAULT,
		systick: IrqHandler::DEFAULT,
	};
}
