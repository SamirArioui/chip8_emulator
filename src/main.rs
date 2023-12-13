use std::fs;

fn main() {
    let program = fs::read("data/INVADERS").expect("Failed to read file");
    println!("{}", program.len());
}
