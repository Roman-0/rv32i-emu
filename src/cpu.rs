use crate::dram::*;
use crate::systembus::*;
pub struct Cpu {
    pub register: [u32; 32],
    pub program_counter: u32,
    pub dram: Dram, //vec of bytes
}

impl Cpu {
    pub fn new(code: Vec<u8>) -> Self {
        let mut register = [0; 32];
        register[2] = DRAM_SIZE; //initialize stack pointer

        Cpu {
            register,
            program_counter: 0,
            dram: code,
        }
    }
    //little endian
    pub fn fetch(&self) -> u32 {
        let index = self.program_counter as usize;
        return (self.dram[index] as u32)
            | ((self.dram[index + 1] as u32) << 8)
            | ((self.dram[index + 2] as u32) << 16)
            | ((self.dram[index + 3] as u32) << 24);
    }

    pub fn decode(&mut self, instruction: u32) {
        let opcode = instruction & 0x7f; //0b1111111, grabs rightmost 7 bits;
        let rd = ((instruction >> 7) & 0x1f) as usize; //0b11111, moves 7 bits and grabs right 5
        let rs1 = ((instruction >> 15) & 0x1f) as usize;
        let rs2 = ((instruction >> 20) & 0x1f) as usize;

        match opcode {
            0x13 => {
                // addi
                let imm = ((instruction & 0xfff00000) as i32 >> 20) as u32;
                self.register[rd] = self.register[rs1].wrapping_add(imm);
            }
            0x33 => {
                // add
                self.register[rd] = self.register[rs1].wrapping_add(self.register[rs2]);
            }
            _ => {
                panic!("not implemented yet: {:?}", opcode);
            }
        }
    }
}
