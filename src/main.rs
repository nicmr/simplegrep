extern crate simplegrep;

use std::env;
use std::process;
use simplegrep::Config;

fn main() {

    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("I'm searching for '{}'", config.query);
    println!("in file {}", config.filename);

    if let Err(e) = simplegrep::run(config){
        println!("Application Error: {}", e);
        process::exit(1);   
    }
}