#[derive(Debug, Clone)]
pub struct BoolArray8 {
    data: u8,
}

impl BoolArray8 {
    pub fn default() -> Self {
        Self { data: 0 }
    }

    pub fn get(&self, index: usize) -> bool {
        self.data & (1 << index) != 0
    }

    pub fn set(&mut self, index: usize, value: bool) {
        if value {
            self.data |= 1 << index;
        } else {
            self.data &= !(1 << index);
        }
    }
}