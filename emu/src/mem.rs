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

impl AddressBus for Mem {
    fn copy_from(&mut self, _other: &Self) {
        todo!("copy from");
    }
    fn read_byte(&self, _address_space: AddressSpace, address: u32) -> u32 {
        match address & 0xFFFFFF {
            0..=0x3FFFFF => self.rom.read_byte(address) as _,
            0xA00000..=0xA03FFF => self.z80.read_byte(address) as _,
            0xA04000..=0xA0FFFF => 0,
            0xA10000..=0xA1001F => self.io.read_byte(address) as _,
            0xA11100..=0xA112FF => self.z80.ctrl_read(address) as _,
            0xC00000..=0xDFFFFF => self.vdp.read(address),
            0xFF0000..=0xFFFFFF => self.ram[address as usize & 0xFFFF] as _,
            _ => todo!("read byte {:X}", address),
        }
    }
    fn read_word(&self, address_space: AddressSpace, address: u32) -> u32 {
        if (0xC00000..=0xDFFFFF).contains(&(address & 0xFFFFFF)) {
            return self.vdp.read(address);
        }
        self.read_byte(address_space, address) << 8
        | self.read_byte(address_space, address + 1)
    }
    fn read_long(&self, address_space: AddressSpace, address: u32) -> u32 {
        self.read_word(address_space, address) << 16
            | self.read_word(address_space, address + 2)
    }
    fn write_byte(&mut self, _address_space: AddressSpace, address: u32, value: u32) {
        match address & 0xFFFFFF {
            0..=0x3FFFFF => {},
            0xA00000..=0xA03FFF => self.z80.write_byte(address, value),
            0xA04000..=0xA0FFFF => {},
            0xA10000..=0xA1001F => self.io.write_byte(address, value),
            0xA11100..=0xA112FF => self.z80.ctrl_write(address, value),
            0xC00000..=0xDFFFFF => eprintln!("TODO: vdp write byte {:X} {:X}", address, value),
            0xFF0000..=0xFFFFFF => {
                self.ram[address as usize & 0xFFFF] = value as u8;
            },
            _ => todo!("write byte {:X} {:X}", address, value),
        }
    }
    fn write_word(&mut self, address_space: AddressSpace, address: u32, value: u32) {
        if (0xC00000..=0xDFFFFF).contains(&(address & 0xFFFFFF)) {
            return self.vdp.write(address, value);
        }
        self.write_byte(address_space, address, value >> 8);
        self.write_byte(address_space, address + 1, value & 0xFF);
    }
    fn write_long(&mut self, address_space: AddressSpace, address: u32, value: u32) {
        self.write_word(address_space, address, value >> 16);
        self.write_word(address_space, address + 2, value & 0xFFFF);
    }
}
