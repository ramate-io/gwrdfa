#![no_std]

use core::fmt::{self, Write};
use core::result::{Result, Result::Err, Result::Ok};

#[no_mangle]
#[inline(never)]
pub fn exit(status: u32) -> ! {
	unsafe {
		core::arch::asm!(
			"mv a0, {0}",      // a0 = pointer to ExitStatus
			"li a7, 93",       // syscall number (93 = exit)
			"ecall",
			in(reg) status,
			options(noreturn)
		);
	}
}

#[no_mangle]
#[inline(never)]
pub fn write(system_id: u32, buffer: &[u8]) -> Result<i32, i32> {
	let ret: i32;
	unsafe {
		core::arch::asm!(
			"ecall",
			in("a7") 64,                   // syscall number for write
			in("a0") system_id,                   // file descriptor
			in("a1") buffer.as_ptr(),      // pointer to buffer
			in("a2") buffer.len(),         // length
			lateout("a3") ret,             // return value (bytes written or -errno)
		);
	}

	if ret < 0 {
		Err(ret)
	} else {
		Ok(ret)
	}
}

pub struct Stdout;

impl Write for Stdout {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		write(1, s.as_bytes()).map(|_| ()).map_err(|_| fmt::Error)
	}
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let _ = core::write!($crate::Stdout, $($arg)*);
    });
}

#[macro_export]
macro_rules! println {
    () => ({
        $crate::print!("\n");
    });
    ($($arg:tt)*) => ({
        $crate::print!($($arg)*);
        $crate::print!("\n");
    });
}

/// Writes to a stack-allocated buffer and antcipates that system will write back into the buffer.
///
/// Returns the status of the operation and the size of the buffer written, i.e.,
/// the slices length corresponding to the bytes written by the system.
/// The system MUST always write fewwer bytes than the original length of the buffer.
///
/// This is generally the preferred means of communication between the program and a system,
/// as it doesn't require dynamic memory allocation or special registers.
#[no_mangle]
#[inline(never)]
pub fn write_channel(system_id: u32, buffer: &mut [u8]) -> Result<(i32, u32), i32> {
	let status_ignored = -1;
	let status: i32;
	let size: u32;
	unsafe {
		core::arch::asm!(
			"ecall",
			in("a7") 33,                   // syscall number for write
			in("a0") system_id,            // file descriptor
			in("a1") buffer.as_ptr(),      // pointer to buffer
			in("a2") buffer.len(),         // length
			in("a3") status_ignored,       // if this isn't reset, the system must have ignored the call
			lateout("a3") status,          // return value (bytes written or -errno)
			lateout("a4") size,            // the
		);
	}

	// This is the immediate status of the channel.
	// It does not necessarily indicate the operation has completed.
	if status < 0 {
		Err(status)
	} else {
		Ok((status, size))
	}
}

/// Reads the status of a channel.
///
/// This has the same call structure as [write_channel],
/// but it does not expect the system to iniate anything.
#[no_mangle]
#[inline(never)]
pub fn read_channel(system_id: u32, buffer: &mut [u8]) -> Result<(i32, u32), i32> {
	let status_ignored = -1;
	let status: i32;
	let size: u32;
	unsafe {
		core::arch::asm!(
			"ecall",
			in("a7") 34,                   // syscall number for write
			in("a0") system_id,            // file descriptor
			in("a1") buffer.as_ptr(),      // pointer to buffer
			in("a2") buffer.len(),         // length
			in("a3") status_ignored,       // if this isn't reset, the system must have ignored the call
			lateout("a3") status,          // return value (bytes written or -errno)
			lateout("a4") size,            // the
		);
	}

	// This is the immediate status of the channel.
	// It does not necessarily indicate the operation has completed.
	if status < 0 {
		Err(status)
	} else {
		Ok((status, size))
	}
}

/// Writes to a channel and blocks until the operation is complete.
pub fn block_on_channel(system_id: u32, buffer: &mut [u8]) -> Result<(i32, u32), i32> {
	let (status, size) = write_channel(system_id, buffer)?;
	if status == 0 {
		Ok((status, size))
	} else {
		loop {
			let (status, size) = read_channel(system_id, buffer)?;
			if status == 0 {
				return Ok((status, size));
			}
		}
	}
}

/// Blocks on a channel and returns the slice of the buffer that was written by the system.
pub fn block_request_channel<'a>(
	system_id: u32,
	buffer: &'a mut [u8],
) -> Result<&'a mut [u8], i32> {
	let (_status, size) = block_on_channel(system_id, buffer)?;
	let written_len = size as usize;
	if written_len > buffer.len() {
		return Err(-2);
	}

	// SAFETY: the system guarantees that it wrote at most `buffer.len()` bytes
	Ok(&mut buffer[..written_len])
}
