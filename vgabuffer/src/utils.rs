use crate::colorcode::ColorCode;
use crate::writer::Writer;
use crate::types::{Buffer, Color};

pub fn print_something() {
    let mut writer =  Writer::new(0, ColorCode::new(Color::Yellow, Color::Black), unsafe { &mut *(0xb8000 as *mut Buffer) });
    writer.write_byte(b'H');
    writer.write_string("ello ");
    writer.write_string("Wörld! Bitches");
}