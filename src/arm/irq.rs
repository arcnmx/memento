use void::Void;
use util::ConstDefault;

extern {
	fn __stack_start(_: Void);

	#[link_name = "__start"]
	pub fn start();

	fn __default_isr_handler();
}

pub const STACK_START: *const Void = __stack_start as *const _;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[allow(raw_pointer_derive)]
pub struct IrqHandler(*const Void);

impl ConstDefault for IrqHandler {
	const DEFAULT: IrqHandler = IrqHandler::new(__default_isr_handler as *const _);
}

impl From<unsafe extern fn()> for IrqHandler {
	fn from(f: unsafe extern fn()) -> Self {
		IrqHandler(f as *const _)
	}
}

impl IrqHandler {
	pub const fn new(v: *const Void) -> Self {
		IrqHandler(v)
	}
}

#[repr(C)]
pub struct VectorTable<E, I> {
	pub exception_handlers: E,
	pub interrupt_handlers: I,
}

unsafe impl<E, I> Sync for VectorTable<E, I> { }
unsafe impl<E, I> Send for VectorTable<E, I> { }

#[inline(always)]
pub unsafe fn irq_disable() {
	asm!("cpsid i" :::: "volatile");
}

#[inline(always)]
pub unsafe fn irq_enable() {
	asm!("cpsie i" :::: "volatile");
}

cfg_if! { if #[cfg(target_cpu = "cortex_m0")] {
	pub unsafe fn irq_clear() {
		super::cpu::nvic::ICER.set(0xffffffff);
		super::cpu::nvic::ICPR.set(0xffffffff);
	}
} else {
	pub unsafe fn irq_clear() {
		for v in &super::cpu::nvic::ICER {
			v.set(0xffffffff);
		}
		for v in &super::cpu::nvic::ICPR {
			v.set(0xffffffff);
		}
	}
} }

#[macro_export]
macro_rules! irq_handler {
	($_i:ident: unsafe fn $i:ident ($sp:ident: $sp_ty:ty) $body:block) => {
		#[allow(dead_code)]
		unsafe fn $i($sp: $sp_ty) {
			unsafe fn _impl() {
				asm!(concat!("
				.thumb_func
				", stringify!($_i), ":
				tst lr, #4
				ite eq
				moveq r0, sp
				mrsne r0, psp
				b $0") ::"i"($i as u32)::"volatile");
			}

			$body
		}

		extern {
			fn $_i();
		}
	};
}

#[macro_export]
macro_rules! asm_fn {
	($_i:ident: unsafe fn $i:ident ($($input:tt)*) { $x:expr }
	$($flags:tt),*
	) => {
		#[allow(dead_code)]
		unsafe fn $_i() {
			asm!(concat!("
			.thumb_func
			", stringify!($i), ":
			", $x
			) ::$($input)*::$($flags,)*"volatile");
		}

		extern {
			fn $i();
		}
	};
}
