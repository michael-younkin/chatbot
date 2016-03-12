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
        seen_input.push(line);
        let i = rng.gen_range(0, seen_input.len());
        println!(">> {}", seen_input[i]);
        print!("User: ");
        stdout.flush().unwrap();
    }
}
