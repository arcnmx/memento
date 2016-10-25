use arch::IrqHandler;
use util::ConstDefault;

#[repr(C)]
pub struct FlashConfig([usize; 4]);

#[repr(C)]
pub struct InterruptVectors {
	pub dma0: IrqHandler, // DMA channel transfer complete
	pub dma1: IrqHandler,
	pub dma2: IrqHandler,
	pub dma3: IrqHandler,
	pub reserved_20: IrqHandler,
	pub ftfa: IrqHandler, // command complete and read collision
	pub pmc: IrqHandler, // low-voltage detect/warning
	pub llwu: IrqHandler, // low leakage wakeup
	pub i2c0: IrqHandler,
	pub i2c1: IrqHandler,
	pub spi0: IrqHandler, // single interrupt for all sources
	pub spi1: IrqHandler,
	pub lpuart0: IrqHandler, // LPUART0 status and error
	pub lpuart1: IrqHandler,
	pub uart2_flexio: IrqHandler, // UART2 or FLEXIO
	pub adc0: IrqHandler,
	pub cmp0: IrqHandler,
	pub tpm0: IrqHandler, // single interrupt for all sources
	pub tpm1: IrqHandler,
	pub tpm2: IrqHandler,
	pub rtc: IrqHandler, // RTC alarm
	pub rtc_seconds: IrqHandler,
	pub pit: IrqHandler,
	pub i2s0: IrqHandler,
	pub usb0: IrqHandler,
	pub dac0: IrqHandler,
	pub reserved_42: [IrqHandler; 2],
	pub lptmr0: IrqHandler,
	pub lcd: IrqHandler,
	pub porta: IrqHandler, // pin detect
	pub portcd: IrqHandler, // single interrupt for c and d
}

impl ConstDefault for FlashConfig {
	const DEFAULT: FlashConfig = FlashConfig([0xffffffff, 0xffffffff, 0xffffffff, 0xffff3ffe]);
}

impl ConstDefault for InterruptVectors {
	const DEFAULT: InterruptVectors = InterruptVectors {
		dma0: IrqHandler::DEFAULT,
		dma1: IrqHandler::DEFAULT,
		dma2: IrqHandler::DEFAULT,
		dma3: IrqHandler::DEFAULT,
		reserved_20: IrqHandler::DEFAULT,
		ftfa: IrqHandler::DEFAULT,
		pmc: IrqHandler::DEFAULT,
		llwu: IrqHandler::DEFAULT,
		i2c0: IrqHandler::DEFAULT,
		i2c1: IrqHandler::DEFAULT,
		spi0: IrqHandler::DEFAULT,
		spi1: IrqHandler::DEFAULT,
		lpuart0: IrqHandler::DEFAULT,
		lpuart1: IrqHandler::DEFAULT,
		uart2_flexio: IrqHandler::DEFAULT,
		adc0: IrqHandler::DEFAULT,
		cmp0: IrqHandler::DEFAULT,
		tpm0: IrqHandler::DEFAULT,
		tpm1: IrqHandler::DEFAULT,
		tpm2: IrqHandler::DEFAULT,
		rtc: IrqHandler::DEFAULT,
		rtc_seconds: IrqHandler::DEFAULT,
		pit: IrqHandler::DEFAULT,
		i2s0: IrqHandler::DEFAULT,
		usb0: IrqHandler::DEFAULT,
		dac0: IrqHandler::DEFAULT,
		reserved_42: [IrqHandler::DEFAULT; 2],
		lptmr0: IrqHandler::DEFAULT,
		lcd: IrqHandler::DEFAULT,
		porta: IrqHandler::DEFAULT,
		portcd: IrqHandler::DEFAULT,
	};
}
