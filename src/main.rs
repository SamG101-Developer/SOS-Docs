#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(SOS_Docs::test_runner)]
#![reexport_test_harness_main = "test_main"]


use core::panic::PanicInfo;
use SOS_Docs::{print, println};


#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    SOS_Docs::test_panic_handler(info)
}


#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();

    loop {}
}


#[test_case]
fn test_assertion() {
    assert_eq!(1, 1);
}
