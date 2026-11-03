pub mod instruction;

mod alu;
mod register;

use crate::cpu::{alu::ALU, instruction::Instruction, register::Register};

pub struct CPU {
    registers: [Register; 4],
    pc: usize,
    carry: bool,
    borrow: bool,
    overflow: bool,
    zero: bool,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            registers: std::array::from_fn(|_| Register::new()),
            pc: 0,
            carry: false,
            borrow: false,
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

        self.write_register(destination, result.value)?;

        self.carry = result.carry;
        self.borrow = result.borrow;
        self.overflow = result.overflow;
        self.zero = result.value == 0;

        Ok(())
    }

    pub fn and_registers(
        &mut self,
        destination: usize,
        source_a: usize,
        source_b: usize,
    ) -> Result<(), String> {
        let a = self.read_register(source_a)?;
        let b = self.read_register(source_b)?;

        let result = ALU::and(a, b);

        self.write_register(destination, result.value)?;

        self.carry = result.carry;
        self.borrow = result.borrow;
        self.overflow = result.overflow;
        self.zero = result.value == 0;

        Ok(())
    }

    pub fn or_registers(
        &mut self,
        destination: usize,
        source_a: usize,
        source_b: usize,
    ) -> Result<(), String> {
        let a = self.read_register(source_a)?;
        let b = self.read_register(source_b)?;

        let result = ALU::or(a, b);

        self.write_register(destination, result.value)?;

        self.carry = result.carry;
        self.borrow = result.borrow;
        self.overflow = result.overflow;
        self.zero = result.value == 0;

        Ok(())
    }

    pub fn xor_registers(
        &mut self,
        destination: usize,
        source_a: usize,
        source_b: usize,
    ) -> Result<(), String> {
        let a = self.read_register(source_a)?;
        let b = self.read_register(source_b)?;

        let result = ALU::xor(a, b);

        self.write_register(destination, result.value)?;

        self.carry = result.carry;
        self.borrow = result.borrow;
        self.overflow = result.overflow;
        self.zero = result.value == 0;

        Ok(())
    }

    pub fn execute(&mut self, instruction: Instruction) -> Result<(), String> {
        match instruction {
            Instruction::Add(operation) => self.add_registers(
                operation.destination,
                operation.source_a,
                operation.source_b,
            ),

            Instruction::Sub(operation) => self.subtract_registers(
                operation.destination,
                operation.source_a,
                operation.source_b,
            ),

            Instruction::And(operation) => self.and_registers(
                operation.destination,
                operation.source_a,
                operation.source_b,
            ),

            Instruction::Or(operation) => self.or_registers(
                operation.destination,
                operation.source_a,
                operation.source_b,
            ),

            Instruction::Xor(operation) => self.xor_registers(
                operation.destination,
                operation.source_a,
                operation.source_b,
            ),
        }
    }

    pub fn run(&mut self, program: &[Instruction]) -> Result<(), String> {
        while let Some(instruction) = program.get(self.pc) {
            self.execute(*instruction)?;

            self.pc += 1;
        }

        Ok(())
    }
}
