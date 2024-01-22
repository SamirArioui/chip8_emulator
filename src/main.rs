use std::fs;

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

struct Display {
    screen: [[u8; 64]; 32],
}

#[derive(Debug)]
struct Chip8 {
    memory: [u8; 4096],
    register: Register,
}

impl Chip8 {
    fn new(offset: usize) -> Self {
        Chip8 {
            memory: [0; 4096],
            register: Register::new(offset),
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
}

fn main() {
    let path = String::from("data/TETRIS");
    let offset = 0x200;
    let mut chip8 = Chip8::new(offset);
    chip8.load_program(path);

    println!("opcode: {:X}", chip8.get_curr_opcode());
    chip8.ld_i();
    println!("register: {:#X?}", chip8.register);
    println!("opcode: {:X}", chip8.get_curr_opcode());
    chip8.call();
    println!("register: {:#X?}", chip8.register);
    println!("opcode: {:X}", chip8.get_curr_opcode());
    chip8.ld_vx_byte();
    println!("register: {:#X?}", chip8.register);
    println!("opcode: {:X}", chip8.get_curr_opcode());
    chip8.ld_vx_byte();
    println!("register: {:#X?}", chip8.register);
    println!("opcode: {:X}", chip8.get_curr_opcode());
    chip8.ret();
    println!("register: {:#X?}", chip8.register);
    println!("opcode: {:X}", chip8.get_curr_opcode());
}
