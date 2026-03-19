#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;
use my_program::{exit, println};

#[no_mangle]
fn _start() -> ! {
	unsafe {
		// set stack pointer
		asm!(
			"la sp, {stack}",
			stack = sym _stack_end
		);
	}
	_main();
}

extern "C" {
	static _stack_end: u32;
}

#[no_mangle]
#[inline(never)]
pub extern "C" fn _main() -> ! {
	let _ = main();
	exit(0);
}

pub fn add(a: u32, b: u32) -> u32 {
	a + b
}

fn main() -> Result<(), ()> {
	let mut j = 0;
	for i in 0..10 {
		assert_eq!(i, i);
		j += i;
		j = add(j, i);
		println!("Hello, world!");
		println!("i: {}, j: {}", i, j);
	}

	Ok(())
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	exit(1);
}
