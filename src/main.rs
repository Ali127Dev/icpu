mod cpu;

use crate::cpu::CPU;

fn main() {
    let mut cpu = CPU::new();

    cpu.registers[0].write(10);
    cpu.registers[1].write(20);

    println!("R0: {}", cpu.registers[0].read());
    println!("R1: {}", cpu.registers[1].read());
}
