pub struct IO {
    registers: [u8; 0x10],
    gamepad: [Gamepad; 2],
}

struct Gamepad(usize);

impl From<usize> for Gamepad {
    fn from(state: usize) -> Self {
        Gamepad(state)
    }
}

impl Gamepad {
    // UDLRABCS

    pub fn read(&self, select: bool) -> u8 {
        let flip = |b: u64| (b * 0x0202020202u64 & 0x010884422010u64) % 1023;
        0
    }
}


impl IO {
    pub fn new() -> Self {
        Self {
            registers: [
                0xA0, // version
                0, // ctrl 1 data
                0, // ctrl 2 data
                0, // exp data
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
            let select = ctrl & 0x40 != 0;
            self.gamepad[address - 1].read(select)
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
