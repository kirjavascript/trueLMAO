pub struct Z80 {
    ram: [u8; 0x2000],
    bus_ack: bool,
    reset: bool,
}

impl Z80 {
    pub fn new() -> Self {
        Self {
            ram: [0; 0x2000],
            bus_ack: false,
            reset: false,
        }
    }

    pub fn read_byte(&self, address: u32) -> u8 {
        self.ram[address as usize & 0x1FFF]
    }

    pub fn write_byte(&mut self, address: u32, value: u32) {
        self.ram[address as usize & 0x1FFF] = value as u8;
    }

    pub fn ctrl_read(&self, address: u32) -> u8 {
        if address & 0xFFFF == 0x1100 {
            !(self.reset && self.bus_ack) as u8
        } else {
            0
        }
    }

    pub fn ctrl_write(&mut self, mut address: u32, value: u32) {
        address &= 0xFFFF;
        self.ram[address as usize & 0x1FFF] = value as u8;
        if address == 0x1100 {
            self.bus_ack = value == 1;
        } else if address == 0x1200 {
            self.reset = value == 1;
        }
    }
}
