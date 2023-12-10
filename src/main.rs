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
    use x86_64::structures::paging::Page;
    use x86_64::VirtAddr;

    println!("Hello World{}", "!");
    SOS_Docs::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };

    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    let page_ptr = page.start_address().as_mut_ptr() as *mut u64;
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

    #[cfg(test)] test_main();
    println!("It did not crash!");
    SOS_Docs::hlt_loop();
}
