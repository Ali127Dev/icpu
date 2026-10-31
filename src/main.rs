mod cpu;

use crate::cpu::CPU;

fn main() {
    let mut cpu = CPU::new();

    cpu.write_register(0, 10).unwrap();
    cpu.write_register(1, 20).unwrap();

    cpu.add_registers(2, 0, 1).unwrap();

    println!("R0: {}", cpu.read_register(0).unwrap());
    println!("R1: {}", cpu.read_register(1).unwrap());
    println!("R2: {}", cpu.read_register(2).unwrap());
}
