#![no_std] 
#![no_main]

extern crate kernel_with_bootloader;

use core::panic::PanicInfo;

/// Panic handler for the OS.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// OS entry point.
/// Calls the kernel entry function.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    kernel_with_bootloader::main();
    loop {}
}