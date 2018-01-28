use m68k::M68k;
use rom::Rom;
use ram::Ram;
use opcodes::*;

use std::io::Error;

#[derive(Debug)]
pub struct Console {
    pub m68k: M68k,
    pub rom: Rom,
    pub ram: Ram,
    // vdp
    // mode
    // region
}

impl Console {
    // pub const RES_WIDTH: u32 = 320;
    // pub const RES_HEIGHT: u32 = 224;

    pub fn new(path: &str) -> Result<Self, Error> {
        Ok(Console {
            m68k: M68k::new(),
            rom: Rom::new(path)?,
            ram: Ram::new(),
        })
    }

    pub fn start(&mut self) {
        self.m68k.pc = self.rom.entry_point();
    }

    pub fn step(&mut self) {
        let opcode = Opcode::next(&self);
        // TODO: cycle counter

        self.m68k.pc += opcode.length;

        println!("{}", opcode.to_string());
        println!("{:?}", opcode);

        match opcode.code {
            Code::Tst => {
                // always Some
                match opcode.dst_mode.as_ref().unwrap() {
                    &Addr { typ: Mode::AbsLong, .. } => {
                        let mut new_cc = self.m68k.cc & 0b10000;

                        match opcode.size.as_ref().unwrap() {
                            &Size::Long => {
                                let value = self.ram.read_long(
                                    opcode.dst_value.unwrap(),
                                );
                                // get msb
                                let msb = value >> 31;
                                // negative
                                if msb == 1 {
                                    new_cc += 0b01000;
                                }
                                // zero
                                else if value == 0 {
                                    new_cc += 0b00100;
                                }
                            },
                            &Size::Word => {
                                let value = self.ram.read_word(
                                    opcode.dst_value.unwrap(),
                                );
                                // get msb
                                let msb = value >> 15;
                                // negative
                                if msb == 1 {
                                    new_cc += 0b01000;
                                }
                                // zero
                                else if value == 0 {
                                    new_cc += 0b00100;
                                }
                            },
                            &Size::Byte => {
                                let value = self.ram.read_byte(
                                    opcode.dst_value.unwrap(),
                                );
                                // get msb
                                let msb = value >> 7;
                                // negative
                                if msb == 1 {
                                    new_cc += 0b01000;
                                }
                                // zero
                                else if value == 0 {
                                    new_cc += 0b00100;
                                }
                            },
                        }

                        self.m68k.cc = new_cc;
                    },
                    _ => { panic!("TST addr mode not supported"); },
                }
            },
            Code::Lea => {
                let reg_num = opcode.dst_mode.unwrap().reg_num.unwrap();

                match opcode.src_mode.as_ref().unwrap() {
                    &Addr { typ: Mode::PCIndirectDisplace, .. } => {
                        let addr = (self.m68k.pc as i64 + opcode.src_ext.unwrap().displace) as u32;
                        self.m68k.addr[reg_num as usize] = addr -2; // for size of instruction maybe ?!
                    },
                    _ => { panic!("LEA addr mode not supported"); },
                }
            },
            Code::Bne => {
                if !self.m68k.z_set() {
                    self.m68k.pc = (self.m68k.pc as i64 + opcode.dst_ext.unwrap().displace) as u32;
                }
            },
            Code::Nop => {},
            _ => {
                eprintln!("{:?} not implemented", opcode.code);
            },
        }


    }
}
