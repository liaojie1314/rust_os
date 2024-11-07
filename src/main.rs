#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

//这个函数将在 panic 发生时被调用
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello World!").unwrap();
    write!(vga_buffer::WRITER.lock(), ",come number: {} {}", 42, 1.336).unwrap();
    write!(vga_buffer::WRITER.lock(), "\n").unwrap();
    println!("https://www.{}.blog", "yuanyuan");
    loop {
        panic!("panic test");
    }
}
