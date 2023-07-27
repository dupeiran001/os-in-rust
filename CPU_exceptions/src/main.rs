#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(CPU_exceptions::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use CPU_exceptions::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    CPU_exceptions::init();

    //unsafe {
    //    *(0xdeadbeef as *mut u8) = 42;
    //};

    fn stack_overflow() {
        stack_overflow();
    }

    stack_overflow();

    #[cfg(test)]
    test_main();

    println!("It dit not crash");
    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    CPU_exceptions::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_ne!(0, 1);
}
