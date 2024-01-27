use chip8_core::Chip8;

fn main() {
    let path = String::from("data/TETRIS");
    let offset = 0x200;
    let mut chip8 = Chip8::new(offset);
    chip8.load_program(path);
    loop {
        println!("Continue(y/n):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Invalide input");
        let input = input.trim();
        let opcode = chip8.get_curr_opcode();
        println!("Current opcode: {:X}", opcode);
        if input == "y" {
            chip8.run_opcode(opcode);
        } else {
            break;
        };
    }
}
