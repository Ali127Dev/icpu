mod cpu;

use crate::cpu::{
    CPU,
    instruction::{BinaryInstruction, Instruction},
};
fn main() {
    let mut cpu = CPU::new();

    if let Err(error) = cpu.write_register(0, 10) {
        eprintln!("{error}");
        return;
    }

    if let Err(error) = cpu.write_register(1, 20) {
        eprintln!("{error}");
        return;
    }

    let program = vec![
        Instruction::Add(BinaryInstruction {
            destination: 2,
            source_a: 0,
            source_b: 1,
        }),
        Instruction::Sub(BinaryInstruction {
            destination: 3,
            source_a: 2,
            source_b: 1,
        }),
        Instruction::Xor(BinaryInstruction {
            destination: 0,
            source_a: 2,
            source_b: 3,
        }),
    ];

    if let Err(error) = cpu.run(&program) {
        eprintln!("Execution failed: {error}");
        return;
    }

    println!("R0 = {}", cpu.read_register(0).unwrap());
    println!("R2 = {}", cpu.read_register(2).unwrap());
    println!("R3 = {}", cpu.read_register(3).unwrap());
}
