
// STEPS:
// 1. parse args for options (research and implement clap external lib/crate)
// 2. read file (use std::fs) 
// 3. extract from file per options
// 4. display results

extern crate clap;
//use clap::{Arg, App, SubCommand};
use clap::clap_app; // clap_app macro

// to do: use macro or clap lib directly?

fn main() {
    let matches = clap_app!(CrabbyPig =>
        (version:"v0.1.0")
        (about: "[ a Rust language Password Idea Generator ]")
        (@arg minimum: -m --min +takes_value "Minimum word length")
        (@arg maximum: -M --max +takes_value "Maximum word length")
        (@arg word_count: -w --wc +takes_value "Total number of words")
        (@arg dicionary: -d --dict +takes_value "Dicionary source file")
        (@arg verbose: -v --verbose "verbose mode")
    )
    .get_matches();

    if matches.is_present("lists") {
        println!("matches present.");
    }
}
