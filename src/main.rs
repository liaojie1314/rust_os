#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
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

entry_point!(kernel_main);

#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use rust_os::memory;
    use x86_64::{structures::paging::Translate, VirtAddr};

    println!("https://www.{}.blog", "yuanyuan");
    rust_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mapper = unsafe { memory::init(phys_mem_offset) };
    let addresses = [
        // the identity-mapped vga buffer page
        0xb8000,
        // some code page
        0x201008,
        // some stack page
        0x0100_0020_1a10,
        // virtual address mapped to physical address 0
        boot_info.physical_memory_offset,
        // not mapped eg:0 -> None
        0,
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = mapper.translate_addr(virt);
        println!("{:?} -> {:?}", virt, phys);
        unsafe {
            let x = *(address as *mut u8);
            println!("{:?} contains {:}", address, x); // 0 -> panic
        }
    }
    /// 调用断点异常 insert an int3 to trigger a breakpoint exception
    /// ```rust
    /// x86_64::instructions::interrupts::int3();
    /// // trigger a page fault exception
    /// unsafe {
    ///     *(0xdeadbeef as *mut u8) = 42;
    /// }
    /// ```
    /// this is for "double fault" demonstration
    /// ```rust
    /// fn stack_overflow() {
    ///     stack_overflow(); // for each recursion, the return address is pushed
    /// }
    /// // trigger a stack overflow
    /// stack_overflow();
    /// ```
    ///
    /// ```rust
    /// let ptr = 0x20453c as *mut u8;
    /// // read from a code page
    /// unsafe {
    ///     let x = *ptr;
    /// }
    /// println!("read worked");
    /// unsafe {
    ///     *ptr = 42;
    /// }
    /// println!("write worked");
    /// ```

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
