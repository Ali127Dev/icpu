mod register;

use crate::cpu::register::Register;

pub struct CPU {
    pub registers: [Register; 4],
}

impl CPU {
    pub fn new() -> Self {
        Self {
            registers: std::array::from_fn(|_| Register::new()),
        }
    }
}
