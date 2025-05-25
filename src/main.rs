use std::{fs, io::Read};

use jvob::json_values_byte_offsets;
use jvob::TrimString;

fn main() {
    let mut file = fs::File::open("./test.json").unwrap();
    let mut json_bytes: Vec<u8> = Vec::new();
    let _ = file.read_to_end(&mut json_bytes);

    let _ = match json_values_byte_offsets(json_bytes) {
        Ok(jo) => {
            for j in jo {
                if j.r#type() == jvob::JType::JString {
                    println!("map: {:?}", j.value().trim_edges_bytes().unwrap());
                } else {
                    println!("map: {:?}", j.value());
                }
            }
        },
        Err(e) => {
            panic!("{}", e)
        }
    };
}
