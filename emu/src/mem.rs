use r68k_emu::ram::AddressBus;
use r68k_emu::ram::AddressSpace;
use crate::rom::ROM;
use crate::io::IO;
use crate::vdp::VDP;
use crate::z80::Z80;

pub struct Mem {
    pub rom: ROM,
    pub io: IO,
    pub vdp: VDP,
    pub ram: [u8; 0x10000],
    pub z80: Z80,
}

impl Mem {
    pub fn new(rom: ROM) -> Self {
        Mem {
            rom,
            io: IO::new(),
            vdp: VDP::new(),
            ram: [0; 0x10000],
            z80: Z80::new(),
        }
    }
}

impl Mem {
    pub fn read_u8(&self, address: u32) -> u32 {
        match address & 0xFFFFFF {
            0..=0x7FFFFF => self.rom.read_byte(address & 0x3FFFFF) as _,
            0x800000..=0x9FFFFF => /* reserved */ 0,
            0xA00000..=0xA03FFF => self.z80.read_byte(address as u16) as _,
            0xA04000..=0xA0FFFF => /* Z80 */ 0,
            0xA10000..=0xA1001F => self.io.read_byte(address) as _,
            0xA10020..=0xA10FFF => /* reserved */ 0,
            0xA11100..=0xA112FF => self.z80.ctrl_read(address) as _,
            0xC00000..=0xDFFFFF => self.vdp.read(address),
            0xFF0000..=0xFFFFFF => self.ram[address as usize & 0xFFFF] as _,
            _ => {
                println!("todo: read byte {:X}", address);
                0
            },
        }
    }
    pub fn read_u16(&self, address: u32) -> u32 {
        if (0xC00000..=0xDFFFFF).contains(&(address & 0xFFFFFF)) {
            return self.vdp.read(address);
        }
        self.read_u8(address) << 8
        | self.read_u8(address + 1)
    }
    pub fn read_u32(&self, address: u32) -> u32 {
        self.read_u16(address) << 16
            | self.read_u16(address + 2)
    }
    pub fn write_u8(&mut self, address: u32, value: u32) {
        match address & 0xFFFFFF {
            0..=0x3FFFFF => {/* ROM */},
            0x400000..=0x9FFFFF => {/* reserved */},
            0xA00000..=0xA03FFF => self.z80.write_byte(address as u16, value as u8),
            0xA04000..=0xA0FFFF => {/* Z80 */},
            0xA10000..=0xA1001F => self.io.write_byte(address, value),
            0xA10020..=0xA10FFF => {/* reserved */},
            0xA11000..=0xA11001 => {/* memory mode register (no-op?) */},
            0xA11002..=0xA110FF => {/* reserved */},
            0xA11100..=0xA112FF => self.z80.ctrl_write(address, value),
            0xA14101..=0xBFFFFF => {/* reserved */},
            0xC00000..=0xDFFFFF => {/* VDP / PSG? */},
            0xFF0000..=0xFFFFFF => {
                self.ram[address as usize & 0xFFFF] = value as u8;
            },
            _ => println!("todo: write byte {:X} {:X}", address, value),
        }
    }
    pub fn write_u16(&mut self, address: u32, value: u32) {
        if (0xC00000..=0xDFFFFF).contains(&(address & 0xFFFFFF)) {
            return VDP::write(self, address, value);
        }
        self.write_u8(address, value >> 8);
        self.write_u8(address + 1, value & 0xFF);
    }
    pub fn write_u32(&mut self, address: u32, value: u32) {
        self.write_u16(address, value >> 16);
        self.write_u16(address + 2, value & 0xFFFF);
    }
}

impl AddressBus for Mem {
    fn copy_from(&mut self, _other: &Self) {
        todo!("copy from");
    }
    #[inline]
    fn read_byte(&self, _address_space: AddressSpace, address: u32) -> u32 {
        self.read_u8(address)
    }
    #[inline]
    fn read_word(&self, _address_space: AddressSpace, address: u32) -> u32 {
        self.read_u16(address)
    }
    #[inline]
    fn read_long(&self, _address_space: AddressSpace, address: u32) -> u32 {
        self.read_u32(address)
    }
    #[inline]
    fn write_byte(&mut self, _address_space: AddressSpace, address: u32, value: u32) {
        self.write_u8(address, value)
    }
    #[inline]
    fn write_word(&mut self, _address_space: AddressSpace, address: u32, value: u32) {
        self.write_u16(address, value)
    }
    #[inline]
    fn write_long(&mut self, _address_space: AddressSpace, address: u32, value: u32) {
        self.write_u32(address, value)
    }
}
