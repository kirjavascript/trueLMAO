pub struct VDP {
    VRAM: [u8; 0x10000],
    CRAM: [u16; 0x40],
    VSRAM: [u16; 0x40],
    registers: [u8; 0x20],
    status: u32,
    control_code: i32,
    control_address: u32,
    control_pending: bool,
    dma_length: i32,
    dma_source: u32,
    dma_fill: u32,
}

enum VDPType {
    VRAM, CRAM, VSRAM,
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
            dma_length: 0,
            dma_source: 0,
            dma_fill: 0,
        }
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

    pub fn write_data_port(&mut self, value: u32) {
        if self.control_code & 1 == 1 {

        }
        self.control_pending = false;
    }

    pub fn write_control_port(&mut self, value: u32) {

    }
}
