use r68k_emu::ram::AddressBus;
use r68k_emu::ram::AddressSpace;
use crate::rom::ROM;
use crate::io::IO;
use crate::vdp::VDP;

pub struct Mem {
    pub rom: ROM,
    pub work_ram: [u8; 0x10000],
    pub z80_ram: [u8; 0x2000],
    pub vdp: VDP,
    pub io: IO,
}

impl Mem {
    pub fn new(rom: ROM) -> Self {
        Mem {
            rom,
            work_ram: [0; 0x10000],
            z80_ram: [0; 0x2000],
            vdp: VDP {},
            io: IO::new(),
        }
    }
}

impl AddressBus for Mem {
    fn copy_from(&mut self, _other: &Self) {
        todo!("copy from");
    }
    fn read_byte(&self, _address_space: AddressSpace, address: u32) -> u32 {
        match address {
            0..=0x3FFFFF => self.rom.read_byte(address) as _,
            0xA00000..=0xA03FFF => self.z80_ram[address as usize & 0x1FFF] as _,
            0xA04000..=0xA0FFFF => 0,
            // 0xA10000..=0xA10001 => {
            //     // version http://www.hacking-cult.org/?r/18/23
            //     0xE0
            // },
            // 0xA10008..=0xA1000D => {
            //     // controller control
            //     0
            // },
            0xA10000..=0xA1001F => self.io.read_byte(address) as _,
            // z80 ctrl
            // vdp
            0xFF0000..=0xFFFFFF => self.work_ram[address as usize & 0xFFFF] as _,
            _ => todo!("read byte {:X}", address),
        }
    }
    fn read_word(&self, address_space: AddressSpace, address: u32) -> u32 {
        match address {
            0..=0xFFFFFE => {
                self.read_byte(address_space, address) << 8
                | self.read_byte(address_space, address + 1)
            },
            _ => todo!("read word {:X}", address),
        }
    }
    fn read_long(&self, address_space: AddressSpace, address: u32) -> u32 {
        match address {
            0..=0xFFFFFC => {
                self.read_word(address_space, address) << 16
                | self.read_word(address_space, address + 2)
            },
            _ => todo!("read long {:X}", address),
        }
    }
    fn write_byte(&mut self, _address_space: AddressSpace, address: u32, value: u32) {
        match address {
            0..=0x3FFFFF => {},
            0xA00000..=0xA03FFF => {
                self.z80_ram[address as usize & 0x1FFF] = value as u8;
            },
            0xA04000..=0xA0FFFF => {},
            0xA10000..=0xA1001F => self.io.write_byte(address, value),
            0xFF0000..=0xFFFFFF => {
                self.work_ram[address as usize & 0xFFFF] = value as u8;
            },
            _ => todo!("write byte {:X} {:X}", address, value),
        }
    }
    fn write_word(&mut self, address_space: AddressSpace, address: u32, value: u32) {
        match address {
            0..=0xFFFFFE => {
                self.write_byte(address_space, address, value >> 8);
                self.write_byte(address_space, address + 1, value & 0xFF);
            },
            _ => todo!("write word {:X} {:X}", address, value),
        }
    }
    fn write_long(&mut self, address_space: AddressSpace, address: u32, value: u32) {
        match address {
            0..=0xFFFFFC => {
                self.write_word(address_space, address, value >> 16);
                self.write_word(address_space, address + 2, value & 0xFFFF);
            },
            _ => todo!("write long {:X} {:X}", address, value),
        }
    }
}
