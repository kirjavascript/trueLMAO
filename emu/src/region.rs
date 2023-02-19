use crate::rom::ROM;
use crate::io::IO;

#[derive(PartialEq)]
pub enum Region {
    US,
    EU,
    JP,
}

impl Region {
    pub fn is_pal(&self) -> bool {
        self == &Region::EU
    }

    pub fn detect(rom: &ROM) -> Self {
        let region_str = rom.region();
        if region_str.contains('U') {
            return Region::US
        } else if region_str.contains('J') {
            return Region::JP
        } else if region_str.contains('E') {
            return Region::EU
        }
        Region::US
    }

    pub fn set_io_region(region: &Region, io: &mut IO) {
        io.registers[0] = match *region {
            Region::US => 0xA1,
            Region::EU => 0xE1,
            Region::JP => 0x21,
        };
    }
}
