#![no_std]
#![no_main]

pub mod pos_core;
pub mod pos_debug;

use core::panic::PanicInfo;
use pos_debug::vga_text;

static HELLO: &[u8] = b"Hello World!\n";

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! 
{
    vga_text::print_ln(HELLO);
    loop {}
}

/// Required panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> !
{
    loop {}
}