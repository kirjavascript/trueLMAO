pub struct IO {
    registers: [u8; 0x10],
    pub gamepad: [Gamepad; 2],
}

pub struct Gamepad(usize);

impl From<usize> for Gamepad {
    fn from(state: usize) -> Self {
        Gamepad(state)
    }
}

impl Gamepad {
    // .. SABCRLDU

    pub fn read(&self, control: u8) -> u8 {
        let select = control & 0x40;
        let latch = control & 0x80;

        let value = if select != 0 {
            self.0 & 0x3F // CBRLDU
        } else {
            (self.0 >> 2) & 0x30 // SA
            | self.0 & 3 // DU
        };

        latch | select | value as u8
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
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            gamepad: [
                0.into(),
                0.into(),
            ],
        }
    }

    pub fn read_byte(&self, mut address: u32) -> u8 {
        address >>= 1;

        if (1u32..=2).contains(&address) {
            let address = address as usize;
            let ctrl = self.registers[address + 3];
            self.gamepad[address - 1].read(ctrl)
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
