#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;
use vga_buffer::{print, println};

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

/// Panic handler for kernel crashes.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Kernel panic: {}", info);
    loop {}
}

/// Entry point for the kernel.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Welcome to the OS Kernel!");
    println!("VGA text buffer working!");
    loop {}
}