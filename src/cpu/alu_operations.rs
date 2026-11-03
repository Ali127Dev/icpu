use crate::{alu::ALU, cpu::CPU};

impl CPU {
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
}
