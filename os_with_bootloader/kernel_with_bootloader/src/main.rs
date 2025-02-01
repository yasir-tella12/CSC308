#![no_std]

mod vga_buffer;
mod panic;
mod interrupts;
mod gdt;
mod memory;

use core::panic::PanicInfo;

/// Kernel entry function.
pub fn main() {
    println!("Welcome to the OS Kernel!");
    println!("Initializing system...");

    gdt::init_gdt();
    interrupts::init_idt();
    unsafe { memory::init_memory_mapper(x86_64::VirtAddr::new(0)) };

    println!("System ready!");
}