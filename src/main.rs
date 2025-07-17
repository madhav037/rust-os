#![feature(custom_test_frameworks)]
#![test_runner(os::test_runner)]
#![no_std]   // Disable the standard library
#![no_main] // Disable the main function provided by the standard library
#![reexport_test_harness_main = "test_main"]


mod vga_buffer;
mod serial;

use core::panic::PanicInfo; // Import the PanicInfo type
// use os::println;


#[no_mangle]  // don't mangle the name of this function
pub extern "C" fn _start() -> ! { 
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    println!("Hello World{}", "!");
    // panic!("This is a panic message!"); // Trigger a panic
    #[cfg(test)]
    test_main();
    loop {}
}


#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}