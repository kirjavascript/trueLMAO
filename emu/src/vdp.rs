use crate::mem::Mem;

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

    pub fn read(&self, mut address: u32) -> u32 {
        address &= 0x1F;

        if (0x4..=0x7).contains(&address) {
            return self.status;
        }

        todo!("vdp read {:X}", address);
        0
    }


    pub fn write(mem: &mut Mem, mut address: u32, value: u32) {
        address &= 0x1F;
        if address < 0x4 {
            VDP::write_data_port(mem, value);
        } else if address < 0x8 {
            VDP::write_control_port(mem, value);
        } else {
            todo!("vdp write {:X} {:X}", address, value);
        }
    }

    pub fn write_data(mem: &mut Mem, target: VDPType, value: u32) {
        let Mem { vdp, .. } = mem;
        match target {
            VDPType::VRAM => {
                vdp.VRAM[vdp.control_address as usize] = ((value >> 8) & 0xff) as _;
                vdp.VRAM[vdp.control_address as usize + 1] = (value & 0xff) as _;
            },
            VDPType::CRAM => {
                vdp.CRAM[((vdp.control_address & 0x7f) >> 1) as usize] = value as _;
            },
            VDPType::VSRAM => {
                vdp.VSRAM[((vdp.control_address & 0x7f) >> 1) as usize] = value as _;
            },
        }
    }

    fn write_data_port(mem: &mut Mem, value: u32) {
        if mem.vdp.control_code & 1 == 1 {
            VDP::write_data(mem, VDPType::from(mem.vdp.control_code & 0xE), value);
        }
        mem.vdp.control_address = (mem.vdp.control_address + mem.vdp.registers[15] as u32) & 0xffff;
        mem.vdp.control_pending = false;

        if mem.vdp.dma_pending {
            mem.vdp.dma_pending = false;
            for _ in 0..mem.vdp.dma_length() {
                mem.vdp.VRAM[mem.vdp.control_address as usize] = (value >> 8) as _;
                mem.vdp.control_address += mem.vdp.registers[15] as u32;
                mem.vdp.control_address &= 0xFFFF;
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
                        VDP::write_data(mem, VDPType::from(mem.vdp.control_code & 0x7), word);
                        mem.vdp.control_address += mem.vdp.registers[15] as u32;
                        mem.vdp.control_address &= 0xffff;
                        println!("DMA write {}", source);
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
