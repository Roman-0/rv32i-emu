use crate::dram::*;
use crate::systembus;
use crate::systembus::*;
pub struct Cpu {
    pub register: [u32; 32],
    pub program_counter: u32,
    pub bus: Bus, //vec of bytes
}

impl Cpu {
    pub fn new(code: Vec<u8>) -> Self {
        let mut register = [0; 32];
        register[2] = DRAM_SIZE; //initialize stack pointer

        Cpu {
            register,
            program_counter: 0,
            bus: Bus::new(code),
        }
    }
    //little endian
    pub fn fetch(&self) -> Result<u32, ()> {
        match self.bus.load_word(self.program_counter) {
            Ok(instruction) => Ok(instruction),
            Err(_) => Err(()),
        }
    }
    pub fn execute(&mut self, instruction: u32) -> Result<(), ()> {
        let opcode = instruction & 0x7f; //0b1111111, grabs rightmost 7 bits;
        let rd = ((instruction >> 7) & 0x1f) as usize; //0b11111, moves 7 bits and grabs right 5
        let rs1 = ((instruction >> 15) & 0x1f) as usize;
        let rs2 = ((instruction >> 20) & 0x1f) as usize;
        let funct3 = (instruction >> 12) & 0x7;
        let funct7 = (instruction >> 25) & 0x7f;

        match opcode {
            //load
            0x03 => {
                //I-type
                let immediate = ((instruction as i32) >> 20) as u32;
                let address = self.register[rs1].wrapping_add(immediate);
                match funct3 {
                    0x0 => {
                        //load byte
                        let val = self.bus.load_byte(address)?;
                        self.register[rd] = val as i8 as i32 as u32;
                    }
                    0x1 => {
                        //load halfword
                        let val = self.bus.load_halfword(address)?;
                        self.register[rd] = val as i16 as i32 as u32;
                    }
                    0x2 => {
                        //load word
                        let val = self.bus.load_word(address)?;
                        self.register[rd] = val;
                    }
                    0x4 => {
                        //load byte unsigned
                        let val = self.bus.load_byte(address)?;
                        self.register[rd] = val;
                    }
                    0x5 => {
                        //load halfword unsigned
                        let val = self.bus.load_halfword(address)?;
                        self.register[rd] = val;
                    }
                    0x6 => {
                        //load word unsigned
                        let val = self.bus.load_word(address)?;
                        self.register[rd] = val;
                    }
                    _ => {}
                }
            }
            0x13 => {
                // addi
                let imm = ((instruction & 0xfff00000) as i32 >> 20) as u32;
                self.register[rd] = self.register[rs1].wrapping_add(imm);
            }
            0x23 => {
                //store
                let immediate = (((instruction & 0xfe000000) as i32 >> 20) as u32)
                    | ((instruction >> 7) & 0x1f);
                let address = self.register[rs1].wrapping_add(immediate);
                let val = match funct3 {
                    0x0 => self.bus.store_byte(address, self.register[rs2] as u8)?,
                    0x1 => self
                        .bus
                        .store_halfword(address, self.register[rs2] as u16)?,
                    0x2 => self.bus.store_word(address, self.register[rs2] as u32)?,
                    _ => {}
                };
            }
            0x33 => {
                // add
                self.register[rd] = self.register[rs1].wrapping_add(self.register[rs2]);
            }
            _ => {
                panic!("not implemented yet: {:?}", opcode);
                return Err(());
            }
        }
        return Ok(());
    }
}
