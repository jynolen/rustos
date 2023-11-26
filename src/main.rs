#![no_std]
#![no_main]


#![feature(custom_test_frameworks)]
#![test_runner(crate::ostest::test_runner)]
#![reexport_test_harness_main = "_test_main"]

#[cfg(test)]
mod ostest;
mod vgabuffer;
mod serial;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);    
    loop {}
}

fn _kernel_main() {
    println!("Kernel Main");
}

#[no_mangle]
#[cfg(not(test))]
pub extern "C" fn _start() -> ! {
    println!("Kernel Entrypoint.");
    loop {}
}

#[no_mangle]
#[cfg(test)]
pub extern "C" fn _start() -> ! {
    serial_println!("Test Entrypoint.");
    _test_main();
    loop {}
}