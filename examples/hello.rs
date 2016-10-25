#![feature(no_std, linkage, lang_items, const_fn, asm, core_intrinsics, core_slice_ext)]
#![no_std]

#[macro_use]
extern crate memento;
#[macro_use]
extern crate stack;
#[macro_use]
extern crate lazy_static;
extern crate spin;
extern crate allocators;
//extern crate alloc_buddy_simple;

use memento::util::ConstDefault;
use memento::arm::{VectorTable, ExceptionVectors, IrqHandler, IrqContext};
use memento::arm::cpu::sys;
use core::sync::atomic::{AtomicUsize, AtomicBool, Ordering};
use spin::Mutex;
use core::mem;

fn main() {
	println!("hello world");

	assert_eq!(svc!(42, 5), 47);

	let data = AtomicUsize::new(0);
	//sched::spawn(|| ());
	/*for x in 0..3 {
		sched::spawn(|| for _ in 0..x {
			println!("hi");
			let data = data.fetch_add(1, Ordering::Relaxed);
			//println!("thread {}: {}", x, data);
		});
	}*/
	loop {
		//println!("main thread: {}", *data.lock());
	}
}

irq_handler! {
	svcall: unsafe fn svcall_imp(ctx: &mut IrqContext) {
		println!("svcall {} {:?}", ctx.svc(), ctx);
		match ctx.svc() {
			0x55 => {
				sched::inner_task(ctx.r0, ctx.r1 as *const _, ctx.r2);
			},
			svc @ 42 => {
				ctx.r0 += svc as usize;
			},
			svc => panic!("unknown svcall {}", svc),
		}
	}
}

static TICKS: AtomicUsize = AtomicUsize::new(0);
fn systick() {
	TICKS.fetch_add(1, Ordering::Relaxed);

	println!("tick");

	// Trigger a pendsv interrupt
	unsafe {
		// um use bit banding pls
		sys::ICSR.set(sys::ICSR.get() | 0x10000000);
	}
}

asm_fn! {
	pendsv_imp: unsafe fn pendsv("i"(sched::sched)) {
		"push {r4 - r11, lr}
		mov r0, sp
		b $0"
	}
}

mod sched {
	use spin::Mutex;
	use stack::{ArrayVec, Vector};
	use core::sync::atomic::{AtomicUsize, Ordering};
	use memento::arm::IrqContext;
	use core::{mem, ptr};

	#[derive(Debug)]
	struct Task {
		sp: *mut (),
	}

	#[derive(Debug)]
	pub struct Context {
		r4: usize,
		r5: usize,
		r6: usize,
		r7: usize,
		r8: usize,
		r9: usize,
		r10: usize,
		r11: usize,
		lr: usize,
		irq: IrqContext,
	}

	static TASK: AtomicUsize = AtomicUsize::new(0);

	lazy_static! {
		static ref TASKS: Mutex<ArrayVec<[Task; 0x10]>> = Mutex::new(ArrayVec::new());
	}
	static STACKS: ([u64; 0], [u8; 0x4000]) = ([0; 0], [0; 0x4000]);
	static STACK_OFFSET: AtomicUsize = AtomicUsize::new(0);

	pub fn spawn<F: FnOnce()>(f: F) {
		fn entry<F: FnOnce()>(f: &mut Option<F>) {
			f.take().unwrap()();
			loop { }
		}
		unsafe {
			new_task(entry::<F>, Some(f));
		}
	}

	pub unsafe fn new_task<T>(entry: fn(_: &mut T), data: T) {
		svc!(0x55, entry as usize, &data as *const T as *const u8, mem::size_of_val(&data));
		mem::forget(data);
	}

	pub unsafe fn inner_task(entry: usize, data: *const u8, len: usize) {
		let stack = STACK_OFFSET.fetch_add(0x400, Ordering::Relaxed);
		let sp = (&stack as *const _ as *mut u8).offset(0x400) as *mut u8;

		assert!(len <= 0x10);
		let arg = sp.offset(-0x10);
		//let sp = sp.offset(-(len as isize));
		//let sp = ((sp as usize) / 0x20 * 0x20) as *mut u8;
		ptr::copy_nonoverlapping(data, arg, len);

		let context = (sp as *mut Context).offset(-1);
		ptr::write(context, Context {
			irq: IrqContext {
				r0: arg as usize,
				x_psr: 0x21000000,
				pc: ((entry as usize) & !1) as *mut _,
				.. mem::zeroed()
			},
			lr: 0xfffffff9, // thread mode, main stack
			.. mem::zeroed()
		});

		let mut tasks = TASKS.lock();
		let mut task = TASK.load(Ordering::Relaxed) % tasks.len();
		tasks.push(Task {
			sp: context as *mut _,
		});
		TASK.store(task, Ordering::Relaxed);
	}

	pub unsafe fn sched(ctx: &mut Context) {
		let mut tasks = TASKS.lock();
		let task = TASK.fetch_add(1, Ordering::Relaxed) % tasks.len();
		tasks[task].sp = ctx as *mut _ as *mut _;
		let task = (task + 1) % tasks.len();

		let sp = tasks[task].sp;
		drop(tasks);

		asm!("
			pop {r4 - r11, lr}
			mov r0, sp
			msr msp, r0
			bx lr
		" ::"{sp}"(sp):"r0":"volatile");
		::core::intrinsics::unreachable();
	}

	pub unsafe fn init() {
		TASKS.lock().push(mem::zeroed());
	}

	#[inline(never)]
	fn stack_overflow() {
		panic!("stack overflow!");
	}

	asm_fn! {
		stack_check_imp: unsafe fn stack_check() {
			"bkpt"
		}
	}
}

#[no_mangle]
#[linkage = "external"]
#[link_section = ".vector_table"]
pub static VECTOR_TABLE: VectorTable<()> = VectorTable {
	exception_handlers: ExceptionVectors {
		svcall: IrqHandler::new(svcall as *mut _),
		systick: IrqHandler::new(systick as *mut _),
		pendsv: IrqHandler::new(pendsv as *mut _),
		.. ConstDefault::DEFAULT
	},
	interrupt_handlers: (),
};

#[lang = "start"]
#[inline(always)]
unsafe fn start(main: *const u8, _argc: isize, _argv: *const *const u8) -> isize {
	//use alloc_buddy_simple::{FreeBlock, initialize_allocator};
	use memento::{arm, start};
	use core::mem;

	start::init_bss();
	start::init_data();

	/*extern {
		fn __heap_start();
		fn __heap_end();
	}

	static mut FREE_LISTS: [*mut FreeBlock; 3] = [0 as *mut _; 3];

	let heap_size = __heap_end as usize - __heap_start as usize;
	initialize_allocator(__heap_start as usize as *mut _, heap_size, &mut FREE_LISTS);
	*/

	arm::irq::irq_disable();
	arm::irq::irq_clear();
	sys::STCSR.set(0x00000007); // Enable SysTick
	sys::STRVR.set(0x50000); // Set SysTick reset value / interval length
	sched::init();
	arm::irq::irq_enable();

	mem::transmute::<_, fn()>(main)();

	0
}

use allocators::{Allocator, Block, AllocatorError};
use core::slice;
use core::cell::UnsafeCell;
pub struct Sbrk<'a> {
	data: UnsafeCell<&'a mut [u8]>,
	offset: AtomicUsize,
}

impl<'a> Sbrk<'a> {
	pub const fn new(data: &'a mut [u8]) -> Self {
		Sbrk {
			data: UnsafeCell::new(data),
			offset: AtomicUsize::new(0),
		}
	}

	pub unsafe fn data(&self) -> &'a mut [u8] {
		self.data_at(self.offset.load(Ordering::Relaxed))
	}

	pub unsafe fn data_at(&self, offset: usize) -> &'a mut [u8] {
		slice::from_raw_parts_mut(self.data_ptr().offset(offset as isize), self.data_len() - offset)
	}

	pub fn data_ptr(&self) -> *mut u8 {
		unsafe {
			(*self.data.get()).as_mut_ptr()
		}
	}

	pub fn data_len(&self) -> usize {
		unsafe {
			(*self.data.get()).len()
		}
	}
}

unsafe impl<'s> Allocator for Sbrk<'s> {
	unsafe fn allocate_raw(&self, size: usize, align: usize) -> Result<Block, AllocatorError> {
		loop {
			let offset = self.offset.load(Ordering::Relaxed);
			let new_offset = (self.data_ptr() as usize + offset + align - 1) / align * align - self.data_ptr() as usize;
			if new_offset + size < self.data_len() {
				if self.offset.compare_and_swap(offset, new_offset, Ordering::Relaxed) == offset {
					return Ok(Block::new(self.data_ptr().offset(new_offset as isize), size, align))
				}
			} else {
				return Err(AllocatorError::OutOfMemory)
			}
		}
	}

	unsafe fn reallocate_raw<'a>(&'a self, block: Block<'a>, new_size: usize) -> Result<Block<'a>, (AllocatorError, Block<'a>)> {
		Err((AllocatorError::OutOfMemory, block))
	}

	unsafe fn deallocate_raw(&self, block: Block) {
		let offset = block.ptr() as usize - self.data_ptr() as usize;
		let old_offset = offset + block.size();
		if self.offset.compare_and_swap(old_offset, offset, Ordering::Relaxed) != old_offset {
			panic!("can only deallocate newest block");
		}
	}
}

pub struct SingleAllocator<'a> {
	data: UnsafeCell<&'a mut [u8]>,
	allocated: AtomicBool,
}

impl<'a> SingleAllocator<'a> {
	pub const fn new(data: &'a mut [u8]) -> Self {
		SingleAllocator {
			data: UnsafeCell::new(data),
			allocated: AtomicBool::new(false),
		}
	}

	pub fn data_ptr(&self) -> *mut u8 {
		unsafe {
			(*self.data.get()).as_mut_ptr()
		}
	}

	pub fn data_len(&self) -> usize {
		unsafe {
			(*self.data.get()).len()
		}
	}
}

unsafe impl<'s> Allocator for SingleAllocator<'s> {
	unsafe fn allocate_raw(&self, size: usize, align: usize) -> Result<Block, AllocatorError> {
		let ptr = (self.data_ptr() as usize + align - 1) / align * align;
		if ptr >= self.data_ptr() as usize + self.data_len() ||
			self.allocated.compare_and_swap(false, true, Ordering::Relaxed) {
			Err(AllocatorError::OutOfMemory)
		} else {
			Ok(Block::new(ptr as *mut _, size, align))
		}
	}

	unsafe fn reallocate_raw<'a>(&'a self, block: Block<'a>, new_size: usize) -> Result<Block<'a>, (AllocatorError, Block<'a>)> {
		Err((AllocatorError::OutOfMemory, block))
	}

	unsafe fn deallocate_raw(&self, block: Block) {
		self.allocated.store(false, Ordering::Relaxed);
	}
}
