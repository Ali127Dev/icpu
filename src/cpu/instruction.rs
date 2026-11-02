pub enum Instruction {
    Add {
        destination: usize,
        source_a: usize,
        source_b: usize,
    },
    Sub {
        destination: usize,
        source_a: usize,
        source_b: usize,
    },
    And {
        destination: usize,
        source_a: usize,
        source_b: usize,
    },
    Or {
        destination: usize,
        source_a: usize,
        source_b: usize,
    },
    Xor {
        destination: usize,
        source_a: usize,
        source_b: usize,
    },
}
