extern crate rand;

use std::io::Read;
use std::io;
use std::collections::HashSet;
use rand::Rng;

fn main() {
    let mut seen_input: Vec<String> = Vec::new();
    let mut rng = rand::thread_rng();
    loop {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).unwrap();
        seen_input.push(buffer);
        let i = rng.gen_range(0, seen_input.len());
        println!("{}", seen_input[i]);
    }
}
