use std::fs;
use std::io;

const RAM_SIZE: usize = 4096;
const NUM_REG: usize = 16;
const STACK_SIZE: usize = 16;
const START_ADDR: u16 = 0x200;
const NUM_KEYS: usize;

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

#[derive(Debug)]
pub struct Chip8 {
    pc: u16,
    ram: [u8; RAM_SIZE],
    screen: [bool; SCREEN_WIDTH * SCREEN_WIDTH],
    v_reg: [u8; NUM_REG],
    i_reg: u16,
    sp: u16,
    stack: [u8; STACK_SIZE],
    keys: [bool; NUM_KEYS],
    dt: u8,
    st: u8,
}

pub impl Chip8 {
    pub fn new() -> Self {
        Self {
            pc: START_ADDR,
            ram: [0; RAM_SIZE],
            screen: [false; SCREEN_WIDTH * SCREEN_WIDTH],
            v_reg: [0; NUM_REG],
            i_reg: 0,
            sp: 0,
            stack: [0; STACK_SIZE],
            keys: [false; NUM_KEYS],
            dt: 0,
            st: 0,
        }
    }

    fn push(&mut self, val: u16) {
        self.stack[self.sp as usize] = val;
        self.sp += 1;
    }

    fn push(&mut self) {
        self.sp -= 1;
        self.stack[self.sp as usize];
    }

    fn load_program(&mut self, path: String) {
        let program = fs::read(path).expect("Failed to read file");
        for (i, item) in program.iter().enumerate() {
            self.ram[i + self.pc] = *item;
        }
    }

    fn get_curr_opcode(&mut self) -> u16 {
        let op1: u16 = (self.ram[self.pc] as u16) << 8;
        let op2: u16 = self.ram[self.pc + 1] as u16;
        op1 | op2
    }

    fn ld_i(&mut self, opcode: u16) {
        self.i_reg = opcode & 0x0FFF;
        self.pc += 2;
    }

    fn call(&mut self, opcode: u16) {
        self.sp += 1;
        self.stack[self.sp] = self.pc;
        self.pc = (opcode & 0x0FFF) as usize;
    }

    fn ld_vx_byte(&mut self, opcode: u16) {
        let vx = ((opcode & 0x0F00) >> 8) as usize;
        let byte = (opcode & 0x00FF) as u8;
        self.v_reg[vx] = byte;
        self.pc += 2;
    }

    fn add_vx_byte(&mut self, opcode: u16) {
        let vx = ((opcode & 0x0F00) >> 8) as usize;
        let byte = (opcode & 0x00FF) as u8;
        self.v_reg[vx] += byte;
        self.pc += 2;
    }

    fn ret(&mut self) {
        self.pc = self.stack[self.sp];
        self.sp -= 1;
        self.pc += 2;
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
