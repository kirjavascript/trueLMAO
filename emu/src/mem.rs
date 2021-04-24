use r68k_emu::ram::AddressBus;
use r68k_emu::ram::AddressSpace;
use crate::rom::Rom;

// file:///home/cake/dev/trueLMAO/target/doc/src/r68k_emu/ram/pagedmem.rs.html#102-140

pub struct Mem {
    pub rom: Rom,
}

impl AddressBus for Mem {
    fn copy_from(&mut self, other: &Self) {
        todo!();
    }
    fn read_byte(&self, _address_space: AddressSpace, address: u32) -> u32 {
        match address {
            0..=0x3FFFFF => self.rom.read_byte(address) as _,
            _ => todo!(),
        }
    }
    fn read_word(&self, _address_space: AddressSpace, address: u32) -> u32 {
        match address {
            0..=0x3FFFFF => self.rom.read_word(address) as _,
            _ => todo!(),
        }
    }
    fn read_long(&self, _address_space: AddressSpace, address: u32) -> u32 {
        match address {
            0..=0x3FFFFF => self.rom.read_long(address),
            _ => todo!(),
        }
    }
    fn write_byte(&mut self, _address_space: AddressSpace, address: u32, value: u32) {
        todo!();
    }
    fn write_word(&mut self, _address_space: AddressSpace, address: u32, value: u32) {
        todo!();
    }
    fn write_long(&mut self, _address_space: AddressSpace, address: u32, value: u32) {
        todo!();
    }
}
