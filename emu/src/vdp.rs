#[allow(non_snake_case)]
pub struct VDP {
    pub VRAM: [u8; 0x10000],
    pub CRAM: [u16; 0x40],
    pub VSRAM: [u16; 0x40],
    registers: [u8; 0x20],
    status: u32,
    control_code: u32,
    control_address: u32,
    control_pending: bool,
    dma_pending: bool,
}

pub enum VDPType {
    VRAM, CRAM, VSRAM,
}

impl From<u32> for VDPType {
    fn from(value: u32) -> Self {
        match value {
            0 | 1 => VDPType::VRAM,
            2 | 3 => VDPType::CRAM,
            4 | 5 => VDPType::VSRAM,
            _ => unreachable!("VDPType {:X}", value),
        }
    }
}

impl VDP {
    pub fn new() -> Self {
        Self {
            VRAM: [0; 0x10000],
            CRAM: [0; 0x40],
            VSRAM: [0; 0x40],
            registers: [0; 0x20],
            status: 0x3400,
            control_code: 0,
            control_address: 0,
            control_pending: false,
            dma_pending: false,
        }
    }

    fn dma_length(&self) -> u32 {
        self.registers[19] as u32 | ((self.registers[20] as u32) << 8)
    }

    pub fn read(&self, mut address: u32) -> u32 {
        address &= 0x1F;

        if (0x4..=0x7).contains(&address) {
            return self.status;
        }

        todo!("vdp read {:X}", address);
        0
    }

    pub fn write(&mut self, mut address: u32, value: u32) {
        address &= 0x1F;
        if address < 0x4 {
            self.write_data_port(value);
        } else if address < 0x8 {
            self.write_control_port(value);
        } else {
            todo!("vdp write {:X} {:X}", address, value);
        }
    }

    pub fn write_data(&mut self, target: VDPType, value: u32) {
        match target {
            VDPType::VRAM => {
                self.VRAM[self.control_address as usize] = ((value >> 8) & 0xff) as _;
                self.VRAM[self.control_address as usize + 1] = (value & 0xff) as _;
            },
            VDPType::CRAM => {
                self.CRAM[((self.control_address & 0x7f) >> 1) as usize] = value as _;
            },
            VDPType::VSRAM => {
                self.VSRAM[((self.control_address & 0x7f) >> 1) as usize] = value as _;
            },
        }
    }

    fn write_data_port(&mut self, value: u32) {
        if self.control_code & 1 == 1 {
            self.write_data(VDPType::from(self.control_code & 0xE), value);
        }
        self.control_address = (self.control_address + self.registers[15] as u32) & 0xffff;
        self.control_pending = false;

        if self.dma_pending {
            self.dma_pending = false;
            for _ in 0..self.dma_length() {
                self.VRAM[self.control_address as usize] = (value >> 8) as _;
                self.control_address += self.registers[15] as u32;
                self.control_address &= 0xFFFF;
            }
        }
    }

    fn write_control_port(&mut self, value: u32) {
        if self.control_pending {
            self.control_code = (self.control_code & 3) | ((value >> 2) & 0x3c);
            self.control_address = (self.control_address & 0x3fff) | ((value & 3) << 14);
            self.control_pending = false;

            if self.control_code & 0x20 > 0 && self.registers[1] & 0x10 > 0 {
                if (self.registers[23] >> 6) == 2 && (self.control_code & 7) == 1 {
                    self.dma_pending = true;
                } else if (self.registers[23] as u32 >> 6) == 3 {
                   todo!("DMA copy");
                } else {

                    let source =
                        ((self.registers[21] as u32) << 1)
                        | ((self.registers[22] as u32) << 9)
                        | ((self.registers[23] as u32) << 17);

                    for _ in 0..self.dma_length() {
                        // let word = mem.read_word(source)
                        // source += 2;
                        // self.write_data(VDPType::from(self.control_code & 0x7), word);
                        // self.control_address += self.registers[15] as _;
                        // self.control_address &= 0xffff;
                        println!("DMA write {}", source);
                    }

                }
            }
        } else {
            if value & 0xc000 == 0x8000 {
                let (register, value) = ((value >> 8) & 0x1F, value & 0xFF);
                if self.registers[1] & 4 > 0 || register <= 10 {
                    self.registers[register as usize] = value as u8;
                }
                self.control_code = 0;
            } else {
                self.control_code = (self.control_code & 0x3c) | ((value >> 14) & 3);
                self.control_address = (self.control_address & 0xc000) | (value & 0x3fff);
                self.control_pending = true;
            }
        }
    }
}
