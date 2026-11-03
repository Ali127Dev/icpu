#[derive(Clone, Copy)]
pub struct BinaryInstruction {
    pub destination: usize,
    pub source_a: usize,
    pub source_b: usize,
}

#[derive(Clone, Copy)]
pub enum Instruction {
    Add(BinaryInstruction),
    Sub(BinaryInstruction),
    And(BinaryInstruction),
    Or(BinaryInstruction),
    Xor(BinaryInstruction),
}
