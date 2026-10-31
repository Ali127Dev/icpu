pub struct ALU;

impl ALU {
    pub fn add(a: u8, b: u8) -> u8 {
        a.wrapping_add(b)
    }

    pub fn subtract(a: u8, b: u8) -> u8 {
        a.wrapping_sub(b)
    }
}
