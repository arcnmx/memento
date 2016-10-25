use core::fmt;

#[repr(usize)]
pub enum Command {
	Open = 0x01,
	Close = 0x02,
	WriteC = 0x03,
	Write0 = 0x04,
	Write = 0x05,
	Read = 0x06,
	ReadC = 0x07,
	IsError = 0x08,
	IsTTY = 0x09,
	Seek = 0x0a,

	FLen = 0x0c,
	TmpNam = 0x0d,
	Remove = 0x0e,
	Rename = 0x0f,
	Clock = 0x10,
	Time = 0x11,
	System = 0x12,
	Errno = 0x13,

	GetCmdline = 0x15,
	Heapinfo = 0x16,

	Elapsed = 0x30,
	TickFreq = 0x31,
}

pub struct Printer;

impl fmt::Write for Printer {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		print(s);
		Ok(())
	}
}

pub fn print(str: &str) {
	for c in str.as_bytes() {
		unsafe {
			command(Command::WriteC, c as *const _ as usize);
		}
	}
}

cfg_if! {
	if #[cfg(any(
		target_cpu = "cortex-m0",
		target_cpu = "cortex-m1",
		target_cpu = "cortex-m3",
	))] {
		pub unsafe fn command(command: Command, message: usize) -> usize {
			let out: usize;
			asm!("bkpt #0xab"
				 : "={r0}" (out)
				 : "{r0}" (command as usize), "{r1}" (message)
				 : "{r0}", "{r1}", "memory");
			out
		}
	} else {
		pub unsafe fn command(command: Command, message: usize) -> usize {
			let out: usize;
			asm!("svc 0x123456"
				: "={r0}" (out)
				: "{r0}" (command as usize), "{r1}" (message)
				: "{r0}", "{r1}", "memory");
			out
		}
	}
}
