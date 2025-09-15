const VGA_WIDTH : u32 = 80;
const VGA_HEIGHT: u32 = 40;
const VGA_BUFFER: *const u8 = 0xb8000 as *mut u8;

static cursor_position : u32 = 0;


pub fn print_ln(s: &[u8])
{

}




