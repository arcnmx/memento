use std::env;

fn main() {
	let target = env::var("TARGET").unwrap();

	let board = target.split('-').nth(1).unwrap();
	let cpu = "cortex-m3";

	println!("cargo:rustc-cfg=target_board=\"{}\"", board);
	println!("cargo:rustc-cfg=target_cpu=\"{}\"", cpu);
}
