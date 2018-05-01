use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {

    let args: Vec<String> = env::args().collect();


    let query = &args[1];
    let filename = &args[2];

    println!("I'm searching for {}", query);
    println!("in file {}", filename);


    let mut f = File::open(filename).expect("Can't find the file");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Encountered an error reading the file");

    println!("With text: \n{}", contents);
}
