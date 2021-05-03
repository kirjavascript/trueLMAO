use crate::mem::Mem;

#[allow(non_snake_case)]
pub struct VDP {
    pub VRAM: [u8; 0x10000],
    pub CRAM: [u16; 0x40],
    pub VSRAM: [u16; 0x40],
    pub registers: [u8; 0x20],
    pub status: u32,
    pub control_code: u32,
    pub control_address: u32,
    pub control_pending: bool,
    pub dma_pending: bool,
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

    pub fn cram_rgb(&self) -> [(u8, u8, u8); 64] {
        let mut rgb = [(0, 0, 0); 64];

        let dupe = |x| (x << 4) | x;

        for (i, color) in self.CRAM.iter().enumerate() {
            let red = color & 0xf;
            let green = (color & 0xf0) >> 4;
            let blue = (color & 0xf00) >> 8;
            rgb[i] = (dupe(red as u8), dupe(green as u8), dupe(blue as u8));
        }

        rgb
    }

    pub fn screen_width(&self) -> usize {
        if self.registers[12] & 0x01 > 0 { 320 } else { 256 }
    }

    pub fn screen_height(&self) -> usize {
        if self.registers[1] & 0x08 > 0 { 240 } else { 224 }
    }

    pub fn hint_counter(&self) -> isize {
        self.registers[0xA] as isize
    }

    fn dma_length(&self) -> u32 {
        self.registers[0x13] as u32 | ((self.registers[0x14] as u32) << 8)
    }

    pub fn read(&self, mut address: u32) -> u32 {
        address &= 0x1F;

        if (0x4..=0x7).contains(&address) {
            return self.status;
        }

        todo!("vdp read {:X}", address);
    }


    pub fn write(mem: &mut Mem, mut address: u32, value: u32) {
        address &= 0x1F;
        if address < 0x4 {
            mem.vdp.write_data_port(value);
        } else if address < 0x8 {
            VDP::write_control_port(mem, value);
        } else {
            todo!("vdp write {:X} {:X}", address, value);
        }
    }

    fn write_data(&mut self, target: VDPType, value: u32) {
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

    fn write_control_port(mem: &mut Mem, value: u32) {
        if mem.vdp.control_pending {
            mem.vdp.control_code = (mem.vdp.control_code & 3) | ((value >> 2) & 0x3c);
            mem.vdp.control_address = (mem.vdp.control_address & 0x3fff) | ((value & 3) << 14);
            mem.vdp.control_pending = false;

            if mem.vdp.control_code & 0x20 > 0 && mem.vdp.registers[1] & 0x10 > 0 {
                if (mem.vdp.registers[23] >> 6) == 2 && (mem.vdp.control_code & 7) == 1 {
                    mem.vdp.dma_pending = true;
                } else if (mem.vdp.registers[23] as u32 >> 6) == 3 {
                   todo!("DMA copy");
                } else {
                    let mut source =
                        ((mem.vdp.registers[21] as u32) << 1)
                        | ((mem.vdp.registers[22] as u32) << 9)
                        | ((mem.vdp.registers[23] as u32) << 17);

                    for _ in 0..mem.vdp.dma_length() {
                        let word = mem.read_u16(source);
                        source += 2;
                        mem.vdp.write_data(VDPType::from(mem.vdp.control_code & 0x7), word);
                        mem.vdp.control_address += mem.vdp.registers[15] as u32;
                        mem.vdp.control_address &= 0xFFFF;
                    }

                }
            }
        } else {
            if value & 0xc000 == 0x8000 {
                let (register, value) = ((value >> 8) & 0x1F, value & 0xFF);
                if mem.vdp.registers[1] & 4 > 0 || register <= 10 {
                    mem.vdp.registers[register as usize] = value as u8;
                }
                mem.vdp.control_code = 0;
            } else {
                mem.vdp.control_code = (mem.vdp.control_code & 0x3c) | ((value >> 14) & 3);
                mem.vdp.control_address = (mem.vdp.control_address & 0xc000) | (value & 0x3fff);
                mem.vdp.control_pending = true;
            }
        }
    }

}
