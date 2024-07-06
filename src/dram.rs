#[allow(unused)]

pub const DRAM_SIZE: u32 = 1024 * 1024 * 128; // 1MiB
pub const DRAM_BASE: u32 = 0x8000_0000; //

pub struct Dram {
    pub dram: Vec<u8>,
}

impl Dram {
    //initializes all of dram and fills first part with code
    pub fn new(code: Vec<u8>) -> Self {
        let mut dram = vec![0; DRAM_SIZE as usize];
        dram.splice(..code.len(), code.iter().cloned());

        Self { dram }
    }

    pub fn load_word(&self, address: u32) -> u32 {
        let index = (address - DRAM_BASE) as usize;
        return (self.dram[index] as u32) //8 bits
            | ((self.dram[index+1] as u32) << 8)  //16 bits
            | ((self.dram[index+2] as u32) << 16) //24 bits
            | ((self.dram[index+3] as u32) << 24); //32 bits
    }
    pub fn load_halfword(&self, address: u32) -> u32 {
        let index = (address - DRAM_BASE) as usize;
        return (self.dram[index] as u32) //8 bits
            | ((self.dram[index+1] as u32) << 8); //16 bits
    }
    pub fn load_byte(&self, address: u32) -> u32 {
        let index = (address - DRAM_BASE) as usize;
        (self.dram[index] as u32)
    }
    pub fn store_word(&mut self, address: u32, value: u32) {
        let index = (address - DRAM_BASE) as usize;
        self.dram[index] = (value & 0xff) as u8; //grabs rightmost byte
        self.dram[index + 1] = ((value >> 8) & 0xff) as u8; //grabs rightmost byte
        self.dram[index + 2] = ((value >> 8) & 0xff) as u8; //grabs rightmost byte
        self.dram[index + 3] = ((value >> 8) & 0xff) as u8; //grabs rightmost byte
    }
    pub fn store_halfword(&mut self, address: u32, value: u16) {
        let index = (address - DRAM_BASE) as usize;
        self.dram[index] = (value & 0xff) as u8; //grabs rightmost byte
        self.dram[index + 1] = ((value >> 8) & 0xff) as u8; //grabs rightmost byte
    }
    pub fn store_byte(&mut self, address: u32, value: u8) {
        let index = (address - DRAM_BASE) as usize;
        self.dram[index] = (value & 0xff) as u8; //grabs rightmost byte
    }
}
