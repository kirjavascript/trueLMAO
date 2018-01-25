#[derive(Debug)]
pub struct Ram {
}

impl Ram {
    pub fn new() -> Self {
        Ram {}
    }

    pub fn read(&self, addr: u32) {
        match addr {
            0x000000...0x200000 => {}, // rom
            0x200000...0xA00000 => {}, // sega reserved
            0xA00000...0xB00000 => {

            }, // i/o
            0xB00000...0xC00000 => {}, // sega reserved
            0xD00000...0xE00000 => {}, // sega reserved
            0xE00000...0xF00000 => {}, // work ram
            _ => panic!("ram read overflow"),
        }
    }
}
