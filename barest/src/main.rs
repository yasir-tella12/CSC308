#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// Panic handler for no_std environments.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// Entry point function (must be marked `#[no_mangle]` to avoid name mangling).
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // VGA framebuffer address for text mode
    let framebuffer = 0xb8000 as *mut u8;

    // Message to display
    const HELLO: &[u8] = b"Hello, world!";

    // Write each character to the framebuffer
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *framebuffer.offset(i as isize * 2) = byte;       // ASCII byte
            *framebuffer.offset(i as isize * 2 + 1) = 0xb;   // Text color (light cyan)
        }
    }

    // Infinite loop to prevent returning
    loop {}
}
