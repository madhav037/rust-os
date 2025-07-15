#![no_std]   // Disable the standard library
#![no_main] // Disable the main function provided by the standard library

mod vga_buffer;

use core::panic::PanicInfo; // Import the PanicInfo type


#[no_mangle]  // don't mangle the name of this function
pub extern "C" fn _start() -> ! { 
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    println!("Hello World{}", "!");
    panic!("This is a panic message!"); // Trigger a panic
    // loop {}
}


#[panic_handler] // Define a custom panic handler
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {} // Infinite loop to halt the program on panic
}

// https://os.phil-opp.com/vga-text-mode/