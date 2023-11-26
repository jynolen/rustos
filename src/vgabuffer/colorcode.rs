
use super::types::Color; 

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
    /*
        ColorCode is 1 byte with XXXXYYYY 
        XXXX 4 Bits for foreground color 
        YYYY 4 Bits for background color
    */
    pub fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}