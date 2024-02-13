#![no_std]
#![no_main]
#![test_runner(SOS_Docs::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]


extern crate alloc;

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use SOS_Docs::{print, println};
use SOS_Docs::task::{Task, executor::Executor, keyboard};

entry_point!(kernel_main);


#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("[!] {}", info);
    SOS_Docs::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    SOS_Docs::test_panic_handler(info)
}


fn kernel_main(boot_info: &'static BootInfo) -> ! {
    // Imports
    use SOS_Docs::allocator;
    use SOS_Docs::memory;
    use x86_64::VirtAddr;

    // Print the booting message and initialize the operating system.
    println!("[+] Booting...");
    SOS_Docs::init();

    // Get the physical memory offset and initialize the memory mapper and frame allocator. This allows for general
    // memory allocation and deallocation.
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("Heap initialised");

    // Call the test_main function if the test feature is enabled.
    #[cfg(test)] test_main();

    // Create a new executor and spawn the keyboard task. Then run the executor.
    let mut executor = Executor::new();
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();

    // Loop forever, waiting for interrupts.
    println!("[+] Boot complete");
    SOS_Docs::hlt_loop();
}
