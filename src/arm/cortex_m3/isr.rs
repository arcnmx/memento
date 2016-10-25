use void::Void;
use util::ConstDefault;
use arm::{irq, IrqHandler, STACK_START, start};
use core::ptr;

pub type VectorTable<I> = irq::VectorTable<ExceptionVectors, I>;

pub extern fn cortex_m0_isr() {
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
	pub initial_sp: *const Void,
	pub reset: IrqHandler,
	pub nmi: IrqHandler,
	pub hard_fault: IrqHandler,
	pub memory_fault: IrqHandler,
	pub bus_fault: IrqHandler,
	pub usage_fault: IrqHandler,
	pub reserved_7: [IrqHandler; 4],
	pub svcall: IrqHandler,
	pub reserved_12: [IrqHandler; 2],
	pub pendsv: IrqHandler,
	pub systick: IrqHandler,
}

impl ConstDefault for ExceptionVectors {
	const DEFAULT: ExceptionVectors = ExceptionVectors {
		initial_sp: STACK_START,
		reset: IrqHandler::new(start as *const _),
		nmi: IrqHandler::DEFAULT,
		hard_fault: IrqHandler::DEFAULT,
		memory_fault: IrqHandler::DEFAULT,
		bus_fault: IrqHandler::DEFAULT,
		usage_fault: IrqHandler::DEFAULT,
		reserved_7: [IrqHandler::DEFAULT; 4],
		svcall: IrqHandler::DEFAULT,
		reserved_12: [IrqHandler::DEFAULT; 2],
		pendsv: IrqHandler::DEFAULT,
		systick: IrqHandler::DEFAULT,
	};
}

#[repr(C)]
#[derive(Debug)]
pub struct IrqContext {
	pub r0: usize,
	pub r1: usize,
	pub r2: usize,
	pub r3: usize,
	pub r12: usize,
	pub lr: usize,
	pub pc: *mut u16,
	pub x_psr: usize,
}

impl IrqContext {
	pub fn sp(&mut self) -> *mut Void {
		unsafe {
			(self as *mut _).offset(1) as *mut _
		}
	}

	pub unsafe fn svc(&self) -> u8 {
		*self.pc.offset(-1) as u8
	}
}

impl ConstDefault for IrqContext {
	const DEFAULT: IrqContext = IrqContext {
		r0: 0,
		r1: 0,
		r2: 0,
		r3: 0,
		r12: 0,
		lr: 0,
		pc: ptr::null_mut(),
		x_psr: 0,
	};
}

impl Default for IrqContext {
	fn default() -> Self {
		ConstDefault::DEFAULT
	}
}
