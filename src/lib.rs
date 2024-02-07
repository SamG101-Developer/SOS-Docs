#![no_std]
#![cfg_attr(test, no_main)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![feature(const_mut_refs)]

extern crate alloc;
use core::panic::PanicInfo;
use core::alloc::Layout;

#[cfg(test)]
use bootloader::{BootInfo, entry_point};

#[cfg(test)]
entry_point!(test_kernel_main);


pub mod interrupts;
pub mod vga_buffer;
pub mod serial;
pub mod gdt;
pub mod memory;
pub mod allocator;
pub mod task;


pub fn init() {
    gdt::init();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

pub fn hlt_loop() -> ! {
    loop { x86_64::instructions::hlt(); }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}


pub trait Testable {
    fn run(&self);
}

impl<T: Fn()> Testable for T {
    fn run(&self) -> () {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}


pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    tests.iter().for_each(|test| test.run());
    exit_qemu(QemuExitCode::Success);
}


pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_print!("[failed]");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    hlt_loop();
}


#[cfg(test)]
fn test_kernel_main(_boot_info: &'static BootInfo) -> ! {
    init();
    test_main();
    hlt_loop();
}


#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info);
}


#[test_case]
fn test_breakpoint_exception() {
    x86_64::instructions::interrupts::int3();
}


pub unsafe trait GlobalAllocTrait {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8;
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout);
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8;
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8;
}


#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    panic!("allocation error: {:?}", layout);
}
