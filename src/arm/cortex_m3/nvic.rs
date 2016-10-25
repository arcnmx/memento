use volatile_cell::VolatileCell;

#[allow(improper_ctypes)]
extern {
	#[link_name = "__NVIC_ISER"]
	pub static ISER: [VolatileCell<usize>; 8];

	#[link_name = "__NVIC_ICER"]
	pub static ICER: [VolatileCell<usize>; 8];

	#[link_name = "__NVIC_ISPR"]
	pub static ISPR: [VolatileCell<usize>; 8];

	#[link_name = "__NVIC_ICPR"]
	pub static ICPR: [VolatileCell<usize>; 8];

	#[link_name = "__NVIC_IPR"]
	pub static IPR: [VolatileCell<usize>; 60];

	#[link_name = "__NVIC_STIR"]
	pub static STIR: VolatileCell<usize>;
}
