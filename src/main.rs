#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(rc_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use rc_os::println;
use core::panic::PanicInfo;


// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rc_os::test_panic_handler(info)
}

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    println!("Hello World{}", "!");
    // panic!("Some panic message");
    
    rc_os::init();
    
    // x86_64::instructions::interrupts::int3();  // invoke a breakpoint exception
    
    
    // unsafe {
    //     *(0xdeadbeef as *mut u8) = 42;    // trigger a page fault
    // }
    
    fn stack_overflow() {
        stack_overflow();    // for each recursion, the return address is pushed
    }
    
    stack_overflow();
    
    #[cfg(test)]
    test_main();
    
    println!("It did not crash!");
    loop {}
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}