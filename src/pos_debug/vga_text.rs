use core::sync::atomic;
use core::sync::atomic::AtomicU32;
use crate::pos_core::math;

const VGA_WIDTH : u32 = 80;
const VGA_HEIGHT: u32 = 40;
const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;

// TODO: should be united to structure
static CURSOR_POSITION_X:AtomicU32 = AtomicU32::new(0);
static CURSOR_POSITION_Y:AtomicU32 = AtomicU32::new(0);

//static CURSOR_POSITION::IVec2A;

fn get_cursor_position() -> u32
{
    let pos:math::IVec2A;

    // TODO: ordering should be reviewed
    let cursor_pos_x = CURSOR_POSITION_X.load(atomic::Ordering::SeqCst);
    let cursor_pos_y = CURSOR_POSITION_Y.load(atomic::Ordering::SeqCst);
    return cursor_pos_x * VGA_WIDTH + cursor_pos_y;
}

fn get_next_cursor_position(new_line:bool) -> u32
{
    // TODO: ordering should be reviewed
    let cursor_pos_x = CURSOR_POSITION_X.load(atomic::Ordering::SeqCst);
    let cursor_pos_y = CURSOR_POSITION_Y.load(atomic::Ordering::SeqCst);

    let mut new_cursor_pos_x = cursor_pos_x;
    let mut new_cursor_pos_y = cursor_pos_y;

    if new_line || cursor_pos_x == VGA_WIDTH - 1
    {
        new_cursor_pos_x = 0;

        if cursor_pos_y == VGA_HEIGHT - 1
        {
            new_cursor_pos_y = 0;
        }
        else
        {
            new_cursor_pos_y = new_cursor_pos_y + 1;
        }
    }

    else
    {
        new_cursor_pos_x = new_cursor_pos_x + 1;
    }

    return new_cursor_pos_x * VGA_WIDTH + new_cursor_pos_y;
}

// TODO: Should be implemented when
fn get_previous_cursor_position(new_line:bool) -> u32
{
    let new_cursor_pos = get_next_cursor_position(new_line);


    return new_cursor_pos;
}

//fn inc_cursor_position() -> u32
//{
    //get_next_cursor_position(false);
//}

//fn dec_cursor_position()
//{
//}

// TODO colors should be added later here
// TODO isn't thread safe
pub fn print(string: &[u8])
{
    for (_i, &byte) in string.iter().enumerate()
    {
        unsafe
            {
                let is_endline = byte == b'\n';

                // TODO: mutex/spinlock should be used, because cursor position can be modified by other threads
                let cursor_position = get_next_cursor_position(is_endline);

                *VGA_BUFFER.offset(cursor_position as isize * 2) = byte;
                *VGA_BUFFER.offset(cursor_position as isize * 2 + 1) = 0xb;
            }
    }
}

// TODO colors should be added later here
// TODO isn't thread safe
pub fn print_ln(string: &[u8])
{
    print(string);
    print(b"\n");
}




