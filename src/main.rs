pub mod memory;
use crate::memory::Ram;
use std::fs;

fn main() {
    let program = fs::read("data/TETRIS").expect("Failed to read file");
    let mut ram = Ram::new();
    ram.write_byte(123, 123);
    println!("{}", ram.read_byte(123));
}
