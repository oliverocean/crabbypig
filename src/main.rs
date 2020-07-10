// main.rs

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("\n> Searching for [ {} ] in file [ {} ]", query, filename);

    let contents = fs::read_to_string(filename)
        .expect("> !! Can't read that file !!");

    println!("\n---[ Contents of {} ]---\n", filename);
    println!("{}", contents);
    println!("\n---[ EOF ]---\n");
}
