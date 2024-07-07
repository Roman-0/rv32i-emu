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

    loop {
        // 1. Fetch.
        let instruction = match cpu.fetch() {
            // Break the loop if an error occurs.
            Ok(inst) => inst,
            Err(_) => break,
        };

        // 2. Add 4 to the program counter.
        cpu.program_counter += 4;

        // 3. Decode.
        // 4. Execute.
        match cpu.execute(instruction) {
            // Break the loop if an error occurs.
            Ok(_) => {}
            Err(_) => break,
        }

        // This is a workaround for avoiding an infinite loop.
        if cpu.program_counter == 0 {
            break;
        }
    }
    dump_registers(&cpu);

    Ok(())
}
