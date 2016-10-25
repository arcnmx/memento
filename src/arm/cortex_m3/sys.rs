use volatile_cell::VolatileCell;

#[allow(improper_ctypes)]
extern {
	#[link_name = "__SYS_ACTLR"]
	pub static ACTLR: VolatileCell<usize>;

	#[link_name = "__SYS_STCSR"]
	pub static STCSR: VolatileCell<usize>;

	#[link_name = "__SYS_STRVR"]
	pub static STRVR: VolatileCell<usize>;

	#[link_name = "__SYS_STCVR"]
	pub static STCVR: VolatileCell<usize>;

	#[link_name = "__SYS_STCR"]
	pub static STCR: VolatileCell<usize>;

	#[link_name = "__SYS_CPUID"]
	pub static CPUID: VolatileCell<usize>;

	#[link_name = "__SYS_ICSR"]
	pub static ICSR: VolatileCell<usize>;

	#[link_name = "__SYS_VTOR"]
	pub static VTOR: VolatileCell<usize>;

	#[link_name = "__SYS_AIRCR"]
	pub static AIRCR: VolatileCell<usize>;

	#[link_name = "__SYS_SCR"]
	pub static SCR: VolatileCell<usize>;

	#[link_name = "__SYS_CCR"]
	pub static CCR: VolatileCell<usize>;

	#[link_name = "__SYS_SHPR"]
	pub static SHPR: [VolatileCell<usize>; 4];

	#[link_name = "__SYS_SHCSR"]
	pub static SHCSR: VolatileCell<usize>;

	#[link_name = "__SYS_CFSR"]
	pub static CFSR: VolatileCell<usize>;

	#[link_name = "__SYS_HFSR"]
	pub static HFSR: VolatileCell<usize>;

	#[link_name = "__SYS_DFSR"]
	pub static DFSR: VolatileCell<usize>;

	#[link_name = "__SYS_MMFAR"]
	pub static MMFAR: VolatileCell<usize>;

	#[link_name = "__SYS_BFAR"]
	pub static BFAR: VolatileCell<usize>;

	#[link_name = "__SYS_AFSR"]
	pub static AFSR: VolatileCell<usize>;

	#[link_name = "__SYS_ID_PFR"]
	pub static ID_PFR: [VolatileCell<usize>; 2];

	#[link_name = "__SYS_ID_DFR0"]
	pub static ID_DFR0: VolatileCell<usize>;

	#[link_name = "__SYS_ID_AFR0"]
	pub static ID_AFR0: VolatileCell<usize>;

	#[link_name = "__SYS_ID_MMFR"]
	pub static ID_MMFR: [VolatileCell<usize>; 4];

	#[link_name = "__SYS_ID_ISAR"]
	pub static ID_ISAR0: [VolatileCell<usize>; 5];

	#[link_name = "__SYS_CPACR"]
	pub static CPACR: VolatileCell<usize>;

	#[link_name = "__SYS_STIR"]
	pub static STIR: VolatileCell<usize>;
}
