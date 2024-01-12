pub mod memory;
use crate::memory::Ram;
use std::fs;

fn main() {
    let program = fs::read("data/TETRIS").expect("Failed to read file");
    let address = 0x3EA - 0x200;
    let inst_p1 = program[address];
    let inst_p2 = program[address + 1];
    println!("{:#X}-{:#X}", inst_p1, inst_p2);
}
