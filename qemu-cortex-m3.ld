ENTRY(__start)
EXTERN(abort VECTOR_TABLE __default_isr_handler __start)
ASSERT(DEFINED(VECTOR_TABLE), "vector table not found")
ASSERT(DEFINED(__default_isr_handler), "__default_isr_handler not found")
ASSERT(DEFINED(__start), "__start not found")

STACK_SIZE = 0x2000;

__NVIC_ISER = 0xE000E100;
__NVIC_ICER = 0xE000E180;
__NVIC_ISPR = 0xE000E200;
__NVIC_ICPR = 0xE000E280;
__NVIC_IABR = 0xE000E300;
__NVIC_IPR = 0xE000E400;
__NVIC_STIR = 0xE000EF00;

__SYS_ICSR = 0xE000ED04;
__SYS_STCSR = 0xE000E010;
__SYS_STRVR = 0xE000E014;

MEMORY
{
	m_vector		(RX) : ORIGIN = 0x00000000, LENGTH = 0x00000400
	m_text			(RX) : ORIGIN = 0x00000400, LENGTH = 0x00040000 - 0x400
	m_data			(RW) : ORIGIN = 0x20000000, LENGTH = 0x00010000
}

SECTIONS
{
	.vector_table :
	{
		. = ALIGN(4);
		*(.vector_table)
		. = ALIGN(4);
	} > m_vector

	.text :
	{
		. = ALIGN(4);
		*(.text)
		*(.text*)
		*(.rodata)
		*(.rodata*)
		. = ALIGN(4);
		__data_rom = .;
	} > m_text

	.bss :
	{
		. = ALIGN(4);
		__bss_start = .;
		*(.bss)
		*(.bss*)
		. = ALIGN(4);
		__bss_end = .;
	} > m_data

	.data :
	{
		. = ALIGN(4);
		__data_start = .;
		*(.data)
		*(.data*)
		. = ALIGN(4);
		__data_end = .;
	} > m_data AT > m_text

	.stack :
	{
		. = ALIGN(8);
		__stack_end = .;
		. += STACK_SIZE;
		__stack_start = .;
	} > m_data

	/*.heap :
	{
		. = ALIGN(8);
		__heap_start = .;
		. = ORIGIN(m_data) + LENGTH(m_data);
		__heap_end = .;
	} > m_data*/

	/DISCARD/ :
	{
		*(.ARM.exidx*)
	}
}
