use volatile_cell::VolatileCell;

extern {
	#[link_name = "__NVIC_ISER"]
	pub static ISER: VolatileCell<usize>;

	#[link_name = "__NVIC_ICER"]
	pub static ICER: VolatileCell<usize>;

	#[link_name = "__NVIC_ISPR"]
	pub static ISPR: VolatileCell<usize>;

	#[link_name = "__NVIC_ICPR"]
	pub static ICPR: VolatileCell<usize>;

	#[link_name = "__NVIC_IPR"]
	pub static IPR: [VolatileCell<usize>; 8];
}
