#![no_std]
#![no_main]

use core::panic::PanicInfo;
use crate::vga::{Writer, ColorInformation, Color};

mod vga;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    halt();
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    print_hello_world();
    halt();
}

fn halt() -> ! { loop {} }

fn print_hello_world() {
    let writer: Writer<25, 80> = Writer::new(ColorInformation::new(Color::White, Color::Black));
    writer.writeln("Hello world!")
}

