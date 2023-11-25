
use volatile::VolatileRef;

use crate::types::{ScreenChar, BUFFER_HEIGHT, BUFFER_WIDTH, Buffer};
use crate::colorcode::ColorCode;
use core::ptr::NonNull;

pub struct Writer {
    pub column_position: usize,
    pub color_code: ColorCode,
    buffer: & 'static mut Buffer<'static>,
}

impl Writer {
    pub fn new(column_position: usize, color_code: ColorCode, buffer:  & 'static mut Buffer<'static>) -> Self {
        Self { column_position, color_code, buffer }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col] = VolatileRef::new(NonNull::<ScreenChar>::new(ScreenChar::new(byte,color_code)));
                self.column_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }

        }
    }
    fn new_line(&mut self) {/* TODO */}
}