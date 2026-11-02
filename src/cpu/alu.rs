pub struct ALU;

pub struct ALUResult {
    pub value: u8,
    pub carry: bool,
    pub borrow: bool,
    pub overflow: bool,
}

impl ALU {
    pub fn add(a: u8, b: u8) -> ALUResult {
        let (value, carry) = a.overflowing_add(b);
        let (_, overflow) = (a as i8).overflowing_add(b as i8);

        ALUResult {
            value,
            carry,
            overflow,
            borrow: false,
        }
    }

    pub fn subtract(a: u8, b: u8) -> ALUResult {
        let (value, borrow) = a.overflowing_sub(b);
        let (_, overflow) = (a as i8).overflowing_sub(b as i8);

        ALUResult {
            value,
            borrow,
            overflow,
            carry: false,
        }
    }
}
