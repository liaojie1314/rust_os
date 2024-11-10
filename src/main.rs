#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rust_os::println;

//这个函数将在 panic 发生时被调用
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info)
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("https://www.{}.blog", "yuanyuan");
    rust_os::init();
    // 调用断点异常 insert an int3 to trigger a breakpoint exception
    x86_64::instructions::interrupts::int3();
    #[cfg(test)]
    test_main();
    println!("It did not crash!");
    loop {}
}
