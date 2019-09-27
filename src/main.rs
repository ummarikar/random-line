extern crate rand;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use rand::seq::SliceRandom;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { panic!("You did not provide a file name"); }
    let mut file = File::open(&args[1]).expect("Can not open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Can not read file");
    let lines: Vec<&str> = contents.lines().collect();
    match lines.choose(&mut rand::thread_rng()) {
        Some(value) => println!("{}", value),
        None        => println!("Could not fetch random line")
    }
}