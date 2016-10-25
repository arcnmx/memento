#![feature(const_fn, associated_consts, asm, core_intrinsics, no_std, lang_items, core_str_ext, linkage, allocator)]
#![no_std]
#![allocator]

extern crate void;

#[macro_use]
extern crate cfg_if;

#[macro_use]
mod macros;
mod lang;

pub mod board;
pub mod start;
pub mod util;
pub mod volatile_cell;

#[macro_use]
#[cfg(target_arch = "arm")] pub mod arm;
#[cfg(target_arch = "arm")] pub use self::arm as target;
