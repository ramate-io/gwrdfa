#![no_std]
#![no_main]
extern crate alloc;
use alloc::string::ToString;
use fuste::println;
use fuste_galloc::Galloc;

#[global_allocator]
static GLOBAL_ALLOCATOR: Galloc = Galloc;

pub fn add(a: u32, b: u32) -> u32 {
	a + b
}

#[fuste::main]
fn main() -> Result<(), ()> {
	let mut j = 0;
	for i in 0..10 {
		assert_eq!(i, i);
		j += i;
		j = add(j, i);
		println!("Hello, world!");
		let s = "Hello, world!".to_string();
		println!("s: {}", s);
		println!("i: {}, j: {}", i, j);
	}

	Ok(())
}
