#![allow(unused)]

use utils::dump_registers;

use crate::cpu::*;
use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::thread::panicking;

mod cpu;
mod dram;
mod systembus;
mod utils;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("no path provided");
    }

    let mut file = File::open(&args[1])?;
    let mut code: Vec<u8> = Vec::new();
    file.read_to_end(&mut code)?;

    let mut cpu = Cpu::new(code);

    while cpu.program_counter < cpu.dram.len() as u32 {
        let instruction = cpu.fetch();
        cpu.program_counter += 4;
        cpu.decode(instruction);
    }
    dump_registers(&cpu);

    Ok(())
}
