#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// OS entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Booting OS...");

    // Directly load the kernel (bootloader handles this)
    loop {}
}

/// Panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
