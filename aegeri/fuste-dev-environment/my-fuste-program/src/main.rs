#![no_std]
#![no_main]
use fuste::println;

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
		println!("i: {}, j: {}", i, j);
	}

	Ok(())
}
