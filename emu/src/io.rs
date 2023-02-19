pub struct IO {
    pub registers: [u8; 0x10],
    pub gamepad: [Gamepad; 2],
}

pub struct Gamepad(usize);

impl From<usize> for Gamepad {
    fn from(state: usize) -> Self {
        Gamepad(state)
    }
}

impl Gamepad {
    pub const START: usize = 1 << 7;
    pub const A: usize = 1 << 6;
    pub const C: usize = 1 << 5;
    pub const B: usize = 1 << 4;
    pub const R: usize = 1 << 3;
    pub const L: usize = 1 << 2;
    pub const D: usize = 1 << 1;
    pub const U: usize = 1;

    pub fn read(&self, register: u8, _control: u8) -> u8 {
        let select = register & 0x40;
        let latch = register & 0x80;

        // TODO: add control

        let value = if select != 0 {
            self.0 & 0x3F // CBRLDU
        } else {
            (self.0 >> 2) & 0x30 // SA
            | self.0 & 3 // DU
        };

        latch | select | !value as u8
    }

    pub fn set(&mut self, value: usize) {
        self.0 = value;
    }
}


impl IO {
    pub fn new() -> Self {
        Self {
            registers: [
                0xA0, // version
                0x7F, // ctrl 1 data
                0x7F, // ctrl 2 data
                0x7F, // exp data
                0, // ctrl 1 ctrl
                0, // ctrl 2 ctrl
                0, // exp control
                0xFF,
                0,
                0,
                0xFF,
                0,
                0,
                0xFF,
                0,
                0,
            ],
            gamepad: [
                0.into(),
                0.into(),
            ],
        }
    }

    pub fn read_byte(&self, address: u32) -> u8 {
        let index = (address as usize >> 1) & 0x1f;

        if (1..=2).contains(&index) {
            let reg = self.registers[index];
            let ctrl = self.registers[index + 3];
            self.gamepad[index - 1].read(reg, ctrl)
        } else {
            self.registers[index & 0xF]
        }
    }

    pub fn write_byte(&mut self, address: u32, value: u32) {
        let index = (address as usize >> 1) & 0x1F;

        if (1..=6).contains(&index) {
            self.registers[index] = value as _;
        }
    }
}
