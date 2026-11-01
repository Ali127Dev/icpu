mod alu;
mod register;

use crate::cpu::{alu::ALU, register::Register};

pub struct CPU {
    pub registers: [Register; 4],
    pub carry: bool,
    pub overflow: bool,
    pub zero: bool,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            registers: std::array::from_fn(|_| Register::new()),
            carry: false,
            overflow: false,
            zero: false,
        }
    }

    pub fn read_register(&self, index: usize) -> Result<u8, String> {
        match self.registers.get(index) {
            Some(register) => Ok(register.read()),
            None => Err(format!("Invalid register index: {index}")),
        }
    }

    pub fn write_register(&mut self, index: usize, value: u8) -> Result<(), String> {
        match self.registers.get_mut(index) {
            Some(register) => {
                register.write(value);
                Ok(())
            }
            None => Err(format!("Invalid register index: {index}")),
        }
    }

    pub fn add_registers(
        &mut self,
        destination: usize,
        source_a: usize,
        source_b: usize,
    ) -> Result<(), String> {
        let a = self.read_register(source_a)?;
        let b = self.read_register(source_b)?;

        let result = ALU::add(a, b);

        self.write_register(destination, result.value)?;

        self.carry = result.carry;
        self.overflow = result.overflow;
        self.zero = result.value == 0;

        Ok(())
    }

    pub fn subtract_registers(
        &mut self,
        destination: usize,
        source_a: usize,
        source_b: usize,
    ) -> Result<(), String> {
        let a = self.read_register(source_a)?;
        let b = self.read_register(source_b)?;

        let result = ALU::subtract(a, b);

        self.write_register(destination, result)?;

        Ok(())
    }
}
