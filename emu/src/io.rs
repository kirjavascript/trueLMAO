pub struct IO {
    registers: [u8; 16],
}

impl IO {
    pub fn new() -> Self {
        Self {
            registers: [
                0xA0, 0x7F, 0x7F, 0x7F, 0, 0, 0, 0xFF, 0, 0, 0xFF, 0, 0, 0xFF, 0, 0
            ],
        }
    }

    pub fn read_byte(&self, mut address: u32) -> u8 {
        address >>= 1;

        if (1u32..=3).contains(&address) {
            // TODO: gamepad
            0
        } else {
            self.registers[address as usize & 0xF]
        }
    }

    pub fn write_byte(&mut self, mut address: u32, value: u32) {
        address >>= 1;

        if (1u32..=6).contains(&address) {
            self.registers[address as usize & 0xF] = value as _;
        }
    }
}
