use std::fs::File;
use std::io;
use std::io::prelude::*;

mod compiler;
use crate::compiler::compile;

fn main() -> io::Result<()> {
    let mut source = String::new();

    let filepath = "/Users/antoni/Documents/rust/sen/attic/source.sen";

    // enter your own filepath here
    let mut file = File::open(filepath)
        .expect(format!("Error: no such file or directory: {}", filepath).as_str());

    file.read_to_string(&mut source)
        .expect("Error: could not read file!");

    match compile(source.chars().collect()) {
        Ok(t) => {
            dbg!(t);
        }
        Err(e) => panic!("{:?}", e),
    }
    Ok(())
}
