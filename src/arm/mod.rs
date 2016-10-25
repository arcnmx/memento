#[macro_use]
pub mod irq;
pub mod eabi;
pub mod semihosting;

pub use self::irq::{IrqHandler, STACK_START, start};
pub use self::cpu::isr::{VectorTable, ExceptionVectors, IrqContext};

#[cfg(target_cpu = "cortex-m0")] pub mod cortex_m0;
#[cfg(target_cpu = "cortex-m0")] pub use self::cortex_m0 as cpu;

#[cfg(target_cpu = "cortex-m3")] pub mod cortex_m3;
#[cfg(target_cpu = "cortex-m3")] pub use self::cortex_m3 as cpu;

#[macro_export]
macro_rules! svc {
	($svc:expr, $r0:expr) => {
		unsafe {
			let out: u32;
			asm!("svc $2" :"={r0}"(out):"{r0}"($r0 as u32), "i"($svc):: "volatile");
			out as usize
		}
	};

	($svc:expr, $r0:expr, $r1:expr, $r2:expr) => {
		unsafe {
			let out: u32;
			asm!("svc $4" :"={r0}"(out):"{r0}"($r0 as u32), "{r1}"($r1 as u32), "{r2}"($r2 as u32), "i"($svc):: "volatile");
			out as usize
		}
	};
}
