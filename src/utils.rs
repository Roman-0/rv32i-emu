use crate::cpu::*;

pub fn dump_registers(cpu: &Cpu) {
    let mut output = String::from("");
    let abi = [
        "zero", " ra ", " sp ", " gp ", " tp ", " t0 ", " t1 ", " t2 ", " s0 ", " s1 ", " a0 ",
        " a1 ", " a2 ", " a3 ", " a4 ", " a5 ", " a6 ", " a7 ", " s2 ", " s3 ", " s4 ", " s5 ",
        " s6 ", " s7 ", " s8 ", " s9 ", " s10", " s11", " t3 ", " t4 ", " t5 ", " t6 ",
    ];
    for i in (0..32).step_by(4) {
        output = format!(
            "{}\n{}",
            output,
            format!(
                "x{:02}({})={:>#18x} x{:02}({})={:>#18x} x{:02}({})={:>#18x} x{:02}({})={:>#18x}",
                i,
                abi[i],
                cpu.register[i],
                i + 1,
                abi[i + 1],
                cpu.register[i + 1],
                i + 2,
                abi[i + 2],
                cpu.register[i + 2],
                i + 3,
                abi[i + 3],
                cpu.register[i + 3],
            )
        );
    }
    println!("{}", output);
}
