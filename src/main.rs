#![no_std]
#![no_main]

pub mod pos_core;
pub mod pos_debug;

use core::panic::PanicInfo;
use pos_debug::vga_text;

static HELLO: &[u8] = b"Hello World\n";
static HELLO2: &[u8] = b"Test string";

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! 
{
    vga_text::print_ln(HELLO);
    vga_text::print_ln(HELLO2);
    loop {}
}

/// Required panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> !
{
    loop {}
}