use std::{fs, io::Read};
use std::env;
use std::process;
use jvob::json_values_byte_offsets;
use jvob::TrimString;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("No file was provided: cargo run -- test.json");
        process::exit(1);
    }
    let file_name = &args[1];
    let mut file = fs::File::open(file_name).unwrap();
    let mut json_bytes: Vec<u8> = Vec::new();
    let _ = file.read_to_end(&mut json_bytes);

    let _ = match json_values_byte_offsets(json_bytes) {
        Ok(jo) => {
            for j in jo {
                if *j.r#type() == jvob::JType::JString {
                    println!("string: {:?}", j.value().trim_edges_bytes().unwrap());
                } else {
                    println!("map: {}", j.value());
                }
            }
        },
        Err(e) => {
            panic!("{}", e)
        }
    };
}
