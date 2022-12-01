use std::fs::File;
use std::io::prelude::*;

pub fn read_file(path: &str) -> Result<String, ()> {
    let mut f = File::open(path).expect("FILE NOT FOUND");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("CONTENT READ ERROR");

    Ok(contents)
}
