use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn read_input(p: &str) -> String {
    // Create a path to the desired file
    // let path = Path::new("src/day1/test.txt");
    let path = Path::new(p);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => s,
    }
}