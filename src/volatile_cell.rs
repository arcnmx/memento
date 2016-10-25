use core::cell::UnsafeCell;
use core::intrinsics::{volatile_load, volatile_store};

pub struct VolatileCell<T>(UnsafeCell<T>);

impl<T> VolatileCell<T> {
	pub unsafe fn get(&self) -> T where T: Copy {
		volatile_load(self.0.get())
	}

	pub unsafe fn set(&self, v: T) {
		volatile_store(self.0.get(), v)
	}
}
