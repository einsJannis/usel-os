#![no_std]
#![no_main]

use core::panic::PanicInfo;

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
    let hello_world: &[u8] = b"Hello World!";
    let vga_buffer = 0xb8000 as *mut u8;
    for (i, &byte) in hello_world.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
}

