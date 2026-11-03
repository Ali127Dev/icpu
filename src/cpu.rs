use crate::register::Register;

mod alu_operations;
mod execution;

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
}
