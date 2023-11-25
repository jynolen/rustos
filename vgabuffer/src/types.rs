
use crate::colorcode::ColorCode;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

impl ScreenChar {
    pub fn new(byte: u8, color_code: ColorCode) -> ScreenChar {
        Self { ascii_character:byte, color_code:color_code }
    }    
}

pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;

use volatile::VolatileRef;

#[repr(transparent)]
pub struct Buffer<'a> {
    //pub chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
    pub chars: [[VolatileRef<'a, ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}