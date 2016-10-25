use core::ptr;

#[no_mangle] #[inline]
pub unsafe extern fn __aeabi_memset(dst: *mut u8, len: usize, v: u32) {
	let v = v as u8;
	for off in 0..len {
		ptr::write(dst.offset(off as isize), v);
	}
}

#[no_mangle] #[inline]
pub unsafe extern fn __aeabi_memset4(dst: *mut u32, len: usize, v: u32) {
	let v = v & 0xff;
	let v = (v << 24) | (v << 16) | (v << 8) | v;
	for off in 0..len / 4 {
		ptr::write(dst.offset(off as isize), v);
	}
}

#[no_mangle] #[inline]
pub unsafe extern fn __aeabi_memset8(dst: *mut u64, len: usize, v: u32) {
	__aeabi_memset4(dst as *mut _, len, v);
}

#[no_mangle] #[inline]
pub unsafe extern fn __aeabi_memclr(dst: *mut u8, len: usize) {
	__aeabi_memset(dst, len, 0);
}

#[no_mangle] #[inline]
pub unsafe extern fn __aeabi_memclr4(dst: *mut u32, len: usize) {
	__aeabi_memset4(dst, len, 0);
}

#[no_mangle] #[inline]
pub unsafe extern fn __aeabi_memclr8(dst: *mut u64, len: usize) {
	__aeabi_memset8(dst, len, 0);
}

#[no_mangle] #[inline]
pub unsafe extern fn __aeabi_memcpy(dst: *mut u8, src: *const u8, len: usize) {
	for off in 0..len {
		ptr::write(dst.offset(off as isize), *src.offset(off as isize));
	}
}

#[no_mangle] #[inline]
pub unsafe extern fn __aeabi_memcpy4(dst: *mut u32, src: *const u32, len: usize) {
	for off in 0..len / 4 {
		ptr::write(dst.offset(off as isize), *src.offset(off as isize));
	}
}

#[no_mangle] #[inline]
pub unsafe extern fn __aeabi_memcpy8(dst: *mut u64, src: *const u64, len: usize) {
	for off in 0..len / 8 {
		ptr::write(dst.offset(off as isize), *src.offset(off as isize));
	}
}

unsafe fn __aeabi_uidivmod() {
	asm!("
	.thumb_func
        .global __aeabi_uidiv
__aeabi_uidiv:

	.thumb_func
        .global __aeabi_uidivmod
__aeabi_uidivmod:
	cmp	r1, #0
	bne	L_no_div0
	bl	__aeabi_idiv0

L_no_div0:
	@ Shift left the denominator until it is greater than the numerator
	movs	r2, #1		@ counter
	movs	r3, #0		@ result
	cmp	r0, r1
	bls	L_sub_loop0
	adds	r1, #0		@ dont shift if denominator would overflow
	bmi	L_sub_loop0
	
L_denom_shift_loop:
	lsls	r2, #1
	lsls	r1, #1
	bmi	L_sub_loop0
	cmp	r0, r1
	bhi	L_denom_shift_loop	
	
L_sub_loop0:	
	cmp	r0, r1
	bcc	L_dont_sub0	@ if (num>denom)

	subs	r0, r0, r1		@ numerator -= denom
	orrs	r3, r2		@ result(r3) |= bitmask(r2)
L_dont_sub0:

	lsrs	r1, #1		@ denom(r1) >>= 1
	lsrs	r2, #1		@ bitmask(r2) >>= 1
	bne	L_sub_loop0

	mov	r1, r0		@ remainder(r1) = numerator(r0)
	mov	r0, r3		@ quotient(r0) = result(r3)
	bx	lr

	.thumb_func
        .global __aeabi_idiv
__aeabi_idiv:

	cmp	r0, #0
	bge	L_num_pos
	
	rsbs	r0, r0, #0		@ num = -num

	cmp	r1, #0
	bge	L_neg_result
	
	rsbs	r1, r1, #0		@ den = -den
	bl	__aeabi_uidivmod
	
L_num_pos:
	cmp	r1, #0
	blt L_no_div
	bl	__aeabi_uidivmod

L_no_div:
	rsbs	r1, r1, #0		@ den = -den

L_neg_result:
	push	{lr}
	bl	__aeabi_uidivmod
	rsbs	r0, r0, #0		@ quot = -quot
	pop	{pc}

	.thumb_func
        .global __aeabi_idiv0
__aeabi_idiv0:

	.thumb_func
        .global __aeabi_ldiv0
__aeabi_ldiv0:
	bx	lr
	"::::"volatile");
}
