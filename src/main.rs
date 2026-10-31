mod cpu;

use crate::cpu::CPU;

fn main() {
    let mut cpu = CPU::new();

    cpu.write_register(0, 10).unwrap();
    cpu.write_register(1, 20).unwrap();

    cpu.subtract_registers(2, 0, 1).unwrap();

    let result = cpu.read_register(2).unwrap();

    println!("Unsigned: {}", result);
    println!("Signed: {}", result as i8);
}
