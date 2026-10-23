pub struct Cpu {
    pub registers: [u8; 16],
    pub pc: usize,
    pub memory: [u8; 4096],
    stack: [u16; 16],
    sp: usize,
    pub steps: usize,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu { registers: [0; 16], pc: 0, memory: [0; 4096], stack: [0; 16], sp: 0, steps: 0 }
    }

    pub fn load(&mut self, addr: usize, program: &[u8]) {
        self.memory[addr..addr + program.len()].copy_from_slice(program);
    }

    pub fn run(&mut self) {
        loop {
            let op = (self.memory[self.pc] as u16) << 8 | self.memory[self.pc + 1] as u16;
            self.pc += 2;
            self.steps += 1;
            match op {
                0x0000 => return,
                _ => panic!("unknown opcode {:04x}", op),
            }
        }
    }
}

pub fn multiply_program(a: u8, b: u8) -> Vec<u8> {
    vec![0x00, 0x00]
}

#[cfg(test)]
mod tests;
