extern crate rand;

use std::io::BufRead;
use std::io::Write;
use std::io;
use std::collections::HashSet;
use rand::Rng;

fn main() {
    let mut seen_input: Vec<String> = Vec::new();
    let mut rng = rand::thread_rng();
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    print!("User: ");
    stdout.flush().unwrap();
    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        seen_input.extend(line.split_whitespace().map(|s| s.to_owned()));
        print!(">> ");
        if seen_input.len() > 0 {
            let response_length = rng.gen_range(3, 15);
            for i in 0..response_length {
                print!("{} ", seen_input[rng.gen_range(0, seen_input.len())]);
            }
            println!("");
        } else {
            println!("...");
        }
        print!("User: ");
        stdout.flush().unwrap();
    }
}
