pub struct Register {
    value: u8,
}

impl Register {
    pub fn new(value: u8) -> Self {
        Self { value }
    }

    pub fn read(&self) -> u8 {
        self.value
    }

    fn write(&mut self, value: u8) {
        self.value = value
    }
}
