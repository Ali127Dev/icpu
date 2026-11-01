mod cpu;

use crate::cpu::CPU;

fn main() {
    let mut cpu = CPU::new();

    cpu.write_register(0, 250).unwrap();
    cpu.write_register(1, 10).unwrap();

    cpu.add_registers(2, 0, 1).unwrap();

    println!("R2: {}", cpu.read_register(2).unwrap());
    println!("Carry: {}", cpu.carry);
    println!("Overflow: {}", cpu.overflow);
    println!("Zero: {}", cpu.zero);
}
