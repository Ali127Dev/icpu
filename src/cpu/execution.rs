use crate::{cpu::CPU, instruction::Instruction};

impl CPU {
    pub fn run(&mut self, program: &[Instruction]) -> Result<(), String> {
        while self.step(program)? {}

        Ok(())
    }

    fn step(&mut self, program: &[Instruction]) -> Result<bool, String> {
        let Some(instruction) = self.fetch(program) else {
            return Ok(false);
        };

        self.execute(instruction)?;

        self.pc += 1;

        Ok(true)
    }

    fn fetch(&self, program: &[Instruction]) -> Option<Instruction> {
        program.get(self.pc).copied()
    }

    fn execute(&mut self, instruction: Instruction) -> Result<(), String> {
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
}
