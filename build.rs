use cargo_metadata::MetadataCommand;
use regex::Regex;
use serde_json::Value;
use std::i16;

fn set_io_base(bootimage_args: &Vec<Value>) {
    for arg in bootimage_args {
        let str_arg = match arg.as_str() {
            None => continue,
            Some(str) => str
        };
        if ! str_arg.contains("iobase"){
            continue;
        }
        let iobase = match Regex::new(r"iobase=0x([0-9a-f]+)").unwrap().captures(str_arg) {
            None => break,
            Some(capture) => capture.get(1).unwrap().as_str()
        };
        let iobase_converted = i16::from_str_radix(iobase, 16).unwrap();
        println!("cargo:rustc-env=IO_DEBUG_BASE={}", iobase_converted)
    }
}

fn main() -> Result<(), String> {  
    for pkg in MetadataCommand::new().exec().unwrap().packages {
        if pkg.name != env!("CARGO_PKG_NAME") {
            continue;
        }
        
        let bootimage_args = match pkg.metadata.get("bootimage") {
            None => continue,
            Some(meta) => match meta.get("test-args") {
                None => continue,
                Some(args) => args.as_array().unwrap()           
            }
        }; 
        set_io_base(bootimage_args);
        let test_return_code = match pkg.metadata.get("bootimage") {
            None => continue,
            Some(meta) => match meta.get("test-success-exit-code") {
                None => continue,
                Some(args) => args.as_i64().unwrap()           
            }
        }; 

        println!("cargo:rustc-env=TEST_RETURN_CODE={}", test_return_code)

    }
    return Ok(());
}