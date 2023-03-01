use z80::{Z80 as Zilog80, Z80_io};

pub struct Z80 {
    cpu: Zilog80<Z80Mem>,
    bus_ack: bool,
    reset: bool,
}

struct Z80Mem {
    ram: [u8; 0x2000],
}

impl Z80_io for Z80Mem {
    fn read_byte(&self, address: u16) -> u8 {
        self.ram[address as usize & 0x1FFF]
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        self.ram[address as usize & 0x1FFF] = value as u8;
    }
}

impl Z80 {
    pub fn new() -> Self {
        Self {
            cpu: Zilog80::new(Z80Mem {
                ram: [0; 0x2000],
            }),
            bus_ack: false,
            reset: false,
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.cpu.io.read_byte(address)
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.cpu.io.write_byte(address, value);
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
        self.cpu.io.ram[address as usize & 0x1FFF] = value as u8;
        if address == 0x1100 {
            self.bus_ack = value == 1;
        } else if address == 0x1200 {
            self.reset = value == 1;
        }
    }
}
