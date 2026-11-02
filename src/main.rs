mod cpu;

use crate::cpu::{CPU, instruction::Instruction};

fn main() {
    let mut cpu = CPU::new();

    cpu.write_register(0, 10).unwrap();
    cpu.write_register(1, 20).unwrap();

    let instruction = Instruction::Add {
        destination: 2,
        source_a: 0,
        source_b: 1,
    };

    if let Err(error) = cpu.execute(instruction) {
        eprintln!("CPU execution error: {error}");
        return;
    }

    println!("R2 = {}", cpu.read_register(2).unwrap());
}
