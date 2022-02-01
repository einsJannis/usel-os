#![no_std]
#![no_main]

use core::panic::PanicInfo;

pub mod driver;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    halt();
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    main();
    halt();
}

fn halt() -> ! { loop {} }

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::driver::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

fn main() {
    println!("Hello World!");
}

