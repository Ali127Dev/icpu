mod cpu;

use crate::cpu::CPU;

fn main() {
    let mut cpu = CPU::new();

    cpu.write_register(0, 10).unwrap();
    cpu.write_register(1, 20).unwrap();

    cpu.subtract_registers(2, 0, 1).unwrap();

    println!("Result: {}", cpu.read_register(2).unwrap());
    println!("Carry: {}", cpu.carry);
    println!("Borrow: {}", cpu.borrow);
    println!("Overflow: {}", cpu.overflow);
    println!("Zero: {}", cpu.zero);
}
