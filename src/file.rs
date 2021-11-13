use std::fs::File;
use std::io::prelude::*;


pub fn open_file(file_path: &mut String) -> String {
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => panic!("file not found: {}", e)
    };
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => (),
        Err(e) => panic!("file not read: {}", e)
    };
    contents
}