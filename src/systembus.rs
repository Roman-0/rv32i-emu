use crate::dram::*;

pub struct Bus {
    dram: Dram,
}

impl Bus {
    pub fn load_word(&self, address: u32) -> Result<u32, ()> {
        if address >= DRAM_BASE {
            return Ok(self.dram.load_word(address));
        }
        Err(())
    }
    pub fn load_halfword(&self, address: u32) -> Result<u32, ()> {
        if address >= DRAM_BASE {
            return Ok(self.dram.load_halfword(address));
        }
        Err(())
    }
    pub fn load_byte(&self, address: u32) -> Result<u32, ()> {
        if address >= DRAM_BASE {
            return Ok(self.dram.load_byte(address));
        }
        Err(())
    }
    pub fn store_word(&mut self, address: u32, value: u32) -> Result<(), ()> {
        if address >= DRAM_BASE {
            self.dram.store_word(address, value);
            return Ok(());
        }
        Err(())
    }
    pub fn store_halfword(&mut self, address: u32, value: u16) -> Result<(), ()> {
        if address >= DRAM_BASE {
            self.dram.store_halfword(address, value);
            return Ok(());
        }
        Err(())
    }
    pub fn store_byte(&mut self, address: u32, value: u8) -> Result<(), ()> {
        if address >= DRAM_BASE {
            self.dram.store_byte(address, value);
            return Ok(());
        }
        Err(())
    }
}
