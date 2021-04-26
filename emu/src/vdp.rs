pub struct VDP {
    VRAM: [u8; 0x10000],
    CRAM: [u16; 0x40],
    VSRAM: [u16; 0x40],
    registers: [u8; 0x20],
}

impl VDP {
    pub fn new() -> Self {
        Self {
            VRAM: [0; 0x10000],
            CRAM: [0; 0x40],
            VSRAM: [0; 0x40],
            registers: [0; 0x20],
        }
    }

    pub fn read(&self, mut address: u32) -> u32 {
        address &= 0xFFFFFF;


        0
    }

    pub fn write(&mut self, mut address: u32, value: u32) {
    }
}
