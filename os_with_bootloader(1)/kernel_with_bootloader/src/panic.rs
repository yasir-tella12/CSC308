use core::panic::PanicInfo;
use crate::println;

/// Custom panic handler that prints an error message and halts the CPU.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("[PANIC] {}", info);
    loop {}
}
