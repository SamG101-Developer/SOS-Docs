#![no_std]
#![no_main]
#![test_runner(SOS_Docs::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]


extern crate alloc;

use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::{vec, vec::Vec};
use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use SOS_Docs::{print, println};
use SOS_Docs::task::{Task, simple_executor::SimpleExecutor};

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
    use SOS_Docs::allocator;
    use SOS_Docs::memory;
    use x86_64::VirtAddr;

    println!("Hello World{}", "!");
    SOS_Docs::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialised");
    let mut executor = SimpleExecutor::new();
    executor.spawn(Task::new(example_task()));
    executor.run();


    #[cfg(test)] test_main();
    println!("It did not crash!");
    SOS_Docs::hlt_loop();
}


async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}
