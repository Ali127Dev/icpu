pub struct Register {
    value: u8,
}

impl Register {
    pub fn new() -> Self {
        Self { value: 0 }
    }

    pub fn read(&self) -> u8 {
        self.value
    }

    pub fn write(&mut self, value: u8) {
        self.value = value
    }
}
