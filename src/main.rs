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
    rust_os::hlt_loop();
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
    // x86_64::instructions::interrupts::int3();
    // trigger a page fault exception
    // unsafe {
    //     *(0xdeadbeef as *mut u8) = 42;
    // }
    // this is for "double fault" demonstration
    // fn stack_overflow() {
    //     stack_overflow(); // for each recursion, the return address is pushed
    // }
    // trigger a stack overflow
    // stack_overflow();

    // let ptr = 0x20453c as *mut u8;
    // // read from a code page
    // unsafe {
    //     let x = *ptr;
    // }
    // println!("read worked");
    // unsafe {
    //     *ptr = 42;
    // }
    // println!("write worked");
    use x86_64::registers::control::Cr3;
    let (level_4_page_table, _) = Cr3::read();
    println!(
        "Level 4 page table at: {:?}",
        level_4_page_table.start_address()
    );

    #[cfg(test)]
    test_main();
    println!("It did not crash!");
    rust_os::hlt_loop();
    // loop {
    //     // deadlock
    //     // use rust_os::print;
    //     // for _ in 0..10000 {}
    //     // print!("-");
    // }
}
