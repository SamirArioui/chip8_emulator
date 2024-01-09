pub struct Ram {
    memory: [u8; 4096],
}

impl Ram {
    pub fn new() -> Ram {
        Ram { memory: [0; 4096] }
    }

    pub fn write_byte(&mut self, address: usize, value: u8) {
        self.memory[address] = value;
    }

    pub fn read_byte(&mut self, address: usize) -> u8 {
        self.memory[address]
    }
}
