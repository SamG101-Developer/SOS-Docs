#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(SOS_Docs::test_runner)]
#![reexport_test_harness_main = "test_main"]


use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use SOS_Docs::{print, println};

entry_point!(kernel_main);


#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    SOS_Docs::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    SOS_Docs::test_panic_handler(info)
}


fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use SOS_Docs::memory;
    use x86_64::VirtAddr;

    println!("Hello World{}", "!");
    SOS_Docs::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut _mapper = unsafe { memory::init(phys_mem_offset) };
    let mut _frame_allocator = unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };

    #[cfg(test)] test_main();
    println!("It did not crash!");
    SOS_Docs::hlt_loop();
}
