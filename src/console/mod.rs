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

    // TODO: move read_word to Console?

    pub fn start(&mut self) {
        self.m68k.pc = self.rom.entry_point();
        self.m68k.addr[7] = self.rom.initial_stack_pointer();
        // TODO: clear RAM?
    }

    pub fn step(&mut self) {
        let opcode = Opcode::next(&self);
        // TODO: cycle counter

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
                    _ => { eprintln!("TST addr mode not supported"); },
                }

                self.m68k.pc += opcode.length;
            },
            Code::Lea => {
                let reg_num = opcode.dst_mode.unwrap().reg_num.unwrap();

                match opcode.src_mode.as_ref().unwrap() {
                    &Addr { typ: Mode::PCIndirectDisplace, .. } => {
                        let addr = (self.m68k.pc as i64 + opcode.src_ext.unwrap().displace) as u32;
                        self.m68k.addr[reg_num as usize] = addr +2; // for size of instruction maybe ?! TODO: confirm behaviour
                    },
                    _ => { eprintln!("LEA addr mode not supported"); },
                }

                self.m68k.pc += opcode.length;
            },
            Code::Bra => {
                self.m68k.displace_pc(opcode.dst_ext.unwrap().displace + 2);
            },
            Code::Beq => {
                if self.m68k.z_set() {
                    self.m68k.displace_pc(opcode.dst_ext.unwrap().displace + 2);
                }
                else {
                    self.m68k.pc += opcode.length;
                }
            },
            Code::Bne => {
                if !self.m68k.z_set() {
                    self.m68k.displace_pc(opcode.dst_ext.unwrap().displace + 2);
                }
                else {
                    self.m68k.pc += opcode.length;
                }
            },
            Code::Movem => {
                //"In the case of a word transfer to either address or data registers, each word is sign extended to 32 bits, and the resulting long word is loaded into the associated register."
                // TODO: write to registers in a different order when reversed?

                // increment addr
                let src_mode = opcode.src_mode.as_ref().unwrap();
                match src_mode {
                    // mem to reg
                    &Addr { typ: Mode::AddrIndirectPostInc, .. } => {
                        let dst_mode = opcode.dst_mode.as_ref().unwrap();
                        if let &Mode::MultiRegister((ref addr, ref data)) = &dst_mode.typ {

                            let reg_num = src_mode.reg_num.unwrap();
                            let op_size = match opcode.size.unwrap() {
                                Size::Word => 2,
                                Size::Long => 4,
                                _ => panic!("this should never happen"),
                            };

                            let mut addr_offset = self.m68k.addr[reg_num as usize];
                            // TODO: get MSB
                            // TODO: check loop order

                            // loop over registers, consuming ram
                            for a_x in addr {
                                let next_long = match op_size {
                                    2 => {
                                        let word = self.ram.read_word(addr_offset);
                                        if (word >> 15) & 1 == 1 {
                                            (word as u32) + 0xFFFF0000
                                        }
                                        else {
                                            word as u32
                                        }
                                    },
                                    4 => self.ram.read_long(addr_offset),
                                    _ => panic!("this should never happen"),
                                };

                                self.m68k.addr[*a_x as usize] = next_long;
                                addr_offset += op_size;
                            }
                            for d_x in data {
                                let next_long = match op_size {
                                    2 => {
                                        let word = self.ram.read_word(addr_offset);
                                        if (word >> 15) & 1 == 1 {
                                            (word as u32) + 0xFFFF0000
                                        }
                                        else {
                                            word as u32
                                        }
                                    },
                                    4 => self.ram.read_long(addr_offset),
                                    _ => panic!("this should never happen"),
                                };

                                self.m68k.data[*d_x as usize] = next_long;
                                addr_offset += op_size;
                            }

                            // increment register
                            let inc = (addr.len() + data.len()) as u32 * op_size;
                            self.m68k.addr[reg_num as usize] += inc as u32;
                        };
                    },
                    // reg to mem
                    // &Addr { typ: Mode::MultiRegister(()), .. } => {
                    _ => { eprintln!("MOVEM addr mode not supported"); },
                }

                self.m68k.pc += opcode.length;
            },
            Code::Nop => {
                self.m68k.pc += opcode.length;
            },
            _ => {
                eprintln!("{:?} not implemented", opcode.code);

                self.m68k.pc += opcode.length;
            },
        }


    }
}
