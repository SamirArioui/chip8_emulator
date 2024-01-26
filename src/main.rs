use std::fs;
use std::io;

#[derive(Debug)]
struct Register {
    v: [u8; 16],
    i: u16,
    pc: usize,
    dt: u8,
    st: u8,
    sp: usize,
    stack: [usize; 16],
}

impl Register {
    fn new(offset: usize) -> Self {
        Register {
            v: [0; 16],
            i: 0,
            pc: offset,
            dt: 0,
            st: 0,
            sp: 0,
            stack: [0; 16],
        }
    }
}

#[derive(Debug)]
struct Display {
    screen: [[u8; 64]; 32],
}

impl Display {
    fn new() -> Self {
        Display {
            screen: [[0; 64]; 32],
        }
    }
}

#[derive(Debug)]
struct Chip8 {
    memory: [u8; 4096],
    register: Register,
    display: Display,
}

impl Chip8 {
    fn new(offset: usize) -> Self {
        Chip8 {
            memory: [0; 4096],
            register: Register::new(offset),
            display: Display::new(),
        }
    }

    fn load_program(&mut self, path: String) {
        let program = fs::read(path).expect("Failed to read file");
        for (i, item) in program.iter().enumerate() {
            self.memory[i + self.register.pc] = *item;
        }
    }

    fn get_curr_opcode(&mut self) -> u16 {
        let op1: u16 = (self.memory[self.register.pc] as u16) << 8;
        let op2: u16 = self.memory[self.register.pc + 1] as u16;
        op1 | op2
    }

    fn ld_i(&mut self, opcode: u16) {
        self.register.i = opcode & 0x0FFF;
        self.register.pc += 2;
    }

    fn call(&mut self, opcode: u16) {
        self.register.sp += 1;
        self.register.stack[self.register.sp] = self.register.pc;
        self.register.pc = (opcode & 0x0FFF) as usize;
    }

    fn ld_vx_byte(&mut self, opcode: u16) {
        let vx = ((opcode & 0x0F00) >> 8) as usize;
        let byte = (opcode & 0x00FF) as u8;
        self.register.v[vx] = byte;
        self.register.pc += 2;
    }

    fn add_vx_byte(&mut self, opcode: u16) {
        let vx = ((opcode & 0x0F00) >> 8) as usize;
        let byte = (opcode & 0x00FF) as u8;
        self.register.v[vx] += byte;
        self.register.pc += 2;
    }

    fn ret(&mut self) {
        self.register.pc = self.register.stack[self.register.sp];
        self.register.sp -= 1;
        self.register.pc += 2;
    }

    fn run_opcode(&mut self, opcode: u16) {
        println!("{:X}", opcode);
        match opcode {
            0x2000..=0x2fff => self.call(opcode),
            0x00ee..=0x00ee => self.ret(),
            0x6000..=0x6fff => self.ld_vx_byte(opcode),
            0xa000..=0xafff => self.ld_i(opcode),
            0x7000..=0x7fff => self.add_vx_byte(opcode),
            _ => println!("UKNOWN INSTRUCTION"),
        }
    }
}

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
