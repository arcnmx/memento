pub mod irq;

use arch::cortex_m3::isr::VectorTable;

#[link_section = ".vector_table"]
#[no_mangle]
pub static VECTOR_TABLE: VectorTable<irq::InterruptVectors> = VectorTable {
	exception_handlers: Default::default(),
	interrupt_handlers: Default::default(),
};
