<div align="center">
  <picture>
    <source srcset="./assets/fuste-bar.png" media="(prefers-color-scheme: dark)">
    <img src="./assets/fuste-bar.png" alt="Robles" width="440">
  </picture>
</div>
</br>

> A pluggable virtual machine for constrained environments.

# Fuste

Fuste is a programmability adapter and virtual machine stack designed for integration with [Robles](https://github.com/ramate-io/robles), [Ramate's](https://github.com/ramate-io/ramate) implementation of [BFA](https://github.com/ramate-io/bfa) protocols. 

> [!NOTE]
> Currently, Fuste implements only the [RV32I](https://docs.riscv.org/reference/isa/unpriv/rv32.html) ISA. Since Fuste is intended as a programmability stack and does not ultimately have general opinions about the ISA, we may choose to implement other ISAs as Fuste virtual machines in the future.

## Getting started 
> [!TIP]
> You can shortcut these steps by cloning this repository, `cd` into [`fuste/tests/toolchain`](/fuste/tests/toolchain/), and `nix develop`. 
>
> **NOTE:** Depending on your system you may still need to build `fubox` beforehand. 

1. Review the programs in the [`tests/toolchain`](/fuste/tests/toolchain/) workspace before you begin writing your own. 
2. Ensure you have built [`fubox`](/fuste/riscv-box/) and that is available on your `PATH`. 
3. Configure a workspace with the desired [`env/fuste`](/fuste/env/fuste/) crates. 
4. Configure the toolchain similar to [`tests/toolchain`](/fuste/tests/toolchain/riscv32i-ramate-fuste-elf.json). You can change the memory layout in the linker script if you like. 
5. Write your program:

```rust 
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
```
6. Run!

```shell
cargo run --target riscv32i-ramate-fuste-elf.json -p my-fuste-program
```

## Environment
[`env/fuste`](/fuste/env/fuste/) provides the following:

- [`fuste-ecall`](/fuste/env/ecall/) for defining basic `ecall` interrupt APIs with an implementing machine.
- [`fuste-exit`](/fuste/env/exit/) for defining program termination interrupts.
- [`fuste-write`](/fuste/env/write/) for making one-way writes to a system. `println!` is implemented using `fuste-write`. 
- [`fuste-channel`](/fuste/env/channel/) for opening a kernel channel with a stack-allocated buffer. Network requests are implemented using `fuste-channel`. 
- [`fuste`](/fuste/env/fuste/) includes all of the above for those who want a complete stack-based set of symbols. 
- [`fuste-galloc`](/fuste/env/galloc/) defines a global heap allocator for those interested in writing heap programs. It is not in [`fuste`](/fuste/env/fuste/) because--owing to the highly constrained targets for the virtual machine--purely stack-based programs are preferred. 

`fubox` currently implements a debugging form of the `fuste` environment. 

## Example programs

- [`my-program`](/fuste/tests/toolchain/my-program/): a program for the `fuste` target without any of the `fuste` prelude
- [`my-fuste-program`](/fuste/tests/toolchain/my-fuste-program/): a simple program using the `fuste` prelude.
- [`my-fuste-heap-program`](/fuste/tests/toolchain/my-fuste-heap-program/): a program using the `fuste` `galloc` dynamic memory allocator. 
- [`my-fuste-dlt-program`](/fuste/tests/toolchain/my-fuste-dlt-program/): a program using the `fuste` DLT primitives.

## Contributing

| Task | Description |
|------|-------------|
| [Upcoming Events](https://github.com/ramate-io/fuste/issues?q=is%3Aissue%20state%3Aopen%20label%3Apriority%3Ahigh%2Cpriority%3Amedium%20label%3Aevent) | High-priority `event` issues with planned completion dates. |
| [Release Candidates](https://github.com/ramate-io/fuste/issues?q=is%3Aissue%20state%3Aopen%20label%3Arelease-candidate) | Feature-complete versions linked to events. |
| [Features & Bugs](https://github.com/ramate-io/fuste/issues?q=is%3Aissue%20state%3Aopen%20label%3Afeature%2Cbug%20label%3Apriority%3Aurgent%2Cpriority%3Ahigh) | High-priority `feature` and `bug` issues. |
