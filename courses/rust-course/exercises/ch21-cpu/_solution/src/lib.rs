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

            let c = (op >> 12) as u8;
            let x = ((op >> 8) & 0xF) as usize;
            let y = ((op >> 4) & 0xF) as usize;
            let d = (op & 0xF) as u8;
            let kk = (op & 0xFF) as u8;
            let nnn = (op & 0x0FFF) as usize;

            match (c, d) {
                _ if op == 0x0000 => return,
                _ if op == 0x00EE => {
                    if self.sp == 0 {
                        panic!("stack underflow");
                    }
                    self.sp -= 1;
                    self.pc = self.stack[self.sp] as usize;
                }
                (0x1, _) => self.pc = nnn,
                (0x2, _) => {
                    if self.sp >= self.stack.len() {
                        panic!("stack overflow");
                    }
                    self.stack[self.sp] = self.pc as u16;
                    self.sp += 1;
                    self.pc = nnn;
                }
                (0x3, _) => {
                    if self.registers[x] == kk {
                        self.pc += 2;
                    }
                }
                (0x4, _) => {
                    if self.registers[x] != kk {
                        self.pc += 2;
                    }
                }
                (0x6, _) => self.registers[x] = kk,
                (0x7, _) => self.registers[x] = self.registers[x].wrapping_add(kk),
                (0x8, 0x0) => self.registers[x] = self.registers[y],
                (0x8, 0x4) => {
                    let (v, carry) = self.registers[x].overflowing_add(self.registers[y]);
                    self.registers[x] = v;
                    self.registers[0xF] = carry as u8;
                }
                (0x8, 0x5) => {
                    let (v, borrow) = self.registers[x].overflowing_sub(self.registers[y]);
                    self.registers[x] = v;
                    self.registers[0xF] = (!borrow) as u8;
                }
                _ => panic!("unknown opcode {:04x}", op),
            }
        }
    }
}

pub fn multiply_program(a: u8, b: u8) -> Vec<u8> {
    vec![
        0x60, 0x00, // 0x000: V0 = 0
        0x61, a,    // 0x002: V1 = a
        0x62, b,    // 0x004: V2 = b (counter)
        0x32, 0x00, // 0x006: skip next if V2 == 0
        0x10, 0x0C, // 0x008: jump to the loop body at 0x00C
        0x00, 0x00, // 0x00A: HALT
        0x80, 0x14, // 0x00C: V0 += V1
        0x72, 0xFF, // 0x00E: V2 -= 1 (adds 255, wrapping)
        0x10, 0x06, // 0x010: jump back to the test
    ]
}

#[cfg(test)]
mod tests;
