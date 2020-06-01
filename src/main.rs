use std::env;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}

fn read_file(path: &str) -> io::Result<()>{
    println!("Opening file a location: {}", path);

    let f = File::open(path)?;
    let f = BufReader::new(f);

    for line in f.lines() {
        println!("{}", line?);
    }

    Ok(())
}
