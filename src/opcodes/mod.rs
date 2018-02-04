use console::Console;
use std::fmt;

#[derive(Debug)]
pub struct Opcode {
    pub code: Code,
    pub length: u32, // in bytes (amount to increase pc) ( change to u32 )
    pub size: Option<Size>, // size of operation
    pub src_mode: Option<Addr>, // effective address
    pub src_value: Option<u32>,
    pub src_ext: Option<Ext>, // extension word
    pub dst_mode: Option<Addr>,
    pub dst_value: Option<u32>,
    pub dst_ext: Option<Ext>,
}

#[derive(Debug, PartialEq)]
pub enum Code {
    Nop, Rts, Illegal,
    Lea,
    Tst, Clr,
    Move, Movem, And,
    Bra, Bne, Beq,
}

impl fmt::Display for Code {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub enum Size { Byte, Word, Long }

#[derive(Debug)]
pub struct Addr {
    pub typ: Mode, // EA
    pub reg_num: Option<u16>,
}

#[derive(Debug, PartialEq)]
pub enum Mode {
    DataDirect, // Dn
    AddrDirect, // An
    AddrIndirect, // (An)
    AddrIndirectPostInc, // (An) +
    AddrIndirectPreInc, // - (An)
    AddrIndirectDisplace, // (d16, An)
    AddrIndirectIndexDisplace, // (d8,An,Xn)
    PCIndirectDisplace, // (d16, PC)
    AbsShort, // (xxx).w
    AbsLong, // (xxx).l
    Immediate, // #<data>
    MultiRegister((Vec<u8>, Vec<u8>)) // a7-d0 (addr, data)
}

#[derive(Debug)]
pub struct Ext {
    pub displace: i64,
    // the following are only used in IndexDisplacement addressing
    pub reg_num: Option<u32>,
    pub reg_type: Option<ExtRegType>,
    pub reg_size: Option<Size>,
}

#[derive(Debug)]
pub enum ExtRegType {
    Data, Addr,
}

impl Opcode {
    fn basic(code: Code) -> Opcode {
        Opcode {
            code,
            length: 2,
            size: None,
            src_mode: None,
            src_value: None,
            src_ext: None,
            dst_mode: None,
            dst_value: None,
            dst_ext: None,
        }
    }

    pub fn next(cn: &Console) -> Self {
        let pc = cn.m68k.pc as usize;
        Opcode::from(cn, pc)
    }

    pub fn from(cn: &Console, pc: usize) -> Self {
        let first_word = cn.rom.read_word(pc);
        let high_byte = first_word & 0xFF00;

        // NOP
        if first_word == 0x4E71 {
            Self::basic(Code::Nop)
        }
        // RTS
        else if first_word == 0x4E75 {
            Self::basic(Code::Rts)
        }
        // ILLEGAL
        else if first_word == 0x4AFC {
            Self::basic(Code::Illegal)
        }
        // LEA
        else if first_word & 0xF1C0 == 0x41C0 {
            let code = Code::Lea;
            let mut length = 2usize;

            let src_mode_ea = (first_word & 0b111000) >> 3;
            let src_mode_reg = first_word & 0b111;
            let src_mode = Self::get_addr_mode(src_mode_ea, src_mode_reg);

            let (src_value, length_inc) = Self::get_value(cn, &src_mode, pc + length, &Size::Long);
            length += length_inc;

            let (src_ext, length_inc) = Self::get_ext_word(cn, &src_mode, pc + length);
            length += length_inc;

            Opcode {
                code,
                length: length as u32,
                size: None,
                src_mode: Some(src_mode),
                src_value,
                src_ext,
                dst_mode: Some(Addr {
                    typ: Mode::AddrDirect,
                    reg_num: Some((first_word & 0xE00) >> 9),
                }),
                dst_value: None,
                dst_ext: None,
            }
        }
        // BRA
        else if high_byte == 0x6000 {
            let code = Code::Bra;
            let mut length = 2usize;

            let (displacement, length_inc) = Self::get_branch_displacement(cn, first_word, pc + length);
            length += length_inc;

            Opcode {
                code,
                length: length as u32,
                size: None,
                src_mode: None,
                src_value: None,
                src_ext: None,
                dst_mode: None,
                dst_value: None,
                dst_ext: Some(Ext {
                    displace: displacement,
                    reg_num: None,
                    reg_size: None,
                    reg_type: None,
                }),
            }
        }
        // BEQ
        else if high_byte == 0x6700 {
            let code = Code::Beq;
            let mut length = 2usize;

            let (displacement, length_inc) = Self::get_branch_displacement(cn, first_word, pc + length);
            length += length_inc;

            Opcode {
                code,
                length: length as u32,
                size: None,
                src_mode: None,
                src_value: None,
                src_ext: None,
                dst_mode: None,
                dst_value: None,
                dst_ext: Some(Ext {
                    displace: displacement,
                    reg_num: None,
                    reg_size: None,
                    reg_type: None,
                }),
            }
        }
        // BNE
        else if high_byte == 0x6600 {
            let code = Code::Bne;
            let mut length = 2usize;

            let (displacement, length_inc) = Self::get_branch_displacement(cn, first_word, pc + length);
            length += length_inc;

            Opcode {
                code,
                length: length as u32,
                size: None,
                src_mode: None,
                src_value: None,
                src_ext: None,
                dst_mode: None,
                dst_value: None,
                dst_ext: Some(Ext {
                    displace: displacement,
                    reg_num: None,
                    reg_size: None,
                    reg_type: None,
                }),
            }
        }
        // MOVE
        else if first_word & 0xC000 == 0 {
            let code = Code::Move;
            let mut length = 2usize;
            let size_bits = (first_word & 0b11000000000000) >> 12;
            let size = Self::get_size(size_bits);

            let src_mode_ea = (first_word & 0b111000) >> 3;
            let src_mode_reg = first_word & 0b111;
            let src_mode = Self::get_addr_mode(src_mode_ea, src_mode_reg);

            let (src_value, length_inc) = Self::get_value(cn, &src_mode, pc + length, &size);
            length += length_inc;

            let (src_ext, length_inc) = Self::get_ext_word(cn, &src_mode, pc + length);
            length += length_inc;

            let dst_mode_reg = (first_word & 0b111000000000) >> 9;
            let dst_mode_ea = (first_word & 0b111000000) >> 6;
            let dst_mode = Self::get_addr_mode(dst_mode_ea, dst_mode_reg);

            let (dst_value, length_inc) = Self::get_value(cn, &dst_mode, pc + length, &size);
            length += length_inc;

            let (dst_ext, length_inc) = Self::get_ext_word(cn, &dst_mode, pc + length);
            length += length_inc;

            Opcode {
                code,
                length: length as u32,
                size: Some(size),
                src_mode: Some(src_mode),
                src_value,
                src_ext,
                dst_mode: Some(dst_mode),
                dst_value,
                dst_ext,
            }
        }
        // MOVEM
        else if first_word & 0xFB80 == 0x4880 {
            let code = Code::Movem;
            let mut length = 2usize;
            let size_bit = (first_word >> 6) & 0b1;
            let size = Self::get_size(size_bit + 1);

            // direction
            let dr_bit = (first_word >> 10) & 0b1;
            // 0 - reg to mem
            // 1 - mem to reg

            let mut registers = cn.rom.read_word(pc + length);
            // a7-d0
            length += 2;

            let first_mode_ea = (first_word & 0b111000) >> 3;
            let first_mode_reg = first_word & 0b111;
            let first_mode = Self::get_addr_mode(first_mode_ea, first_mode_reg);


            let (first_value, length_inc_1) = Self::get_value(cn, &first_mode, pc + length, &size);
            length += length_inc_1;
            let (first_ext, length_inc_2) = Self::get_ext_word(cn, &first_mode, pc + length);
            length += length_inc_2;

            // flip order for -(Xn)
            if first_mode.typ == Mode::AddrIndirectPreInc {
                registers = rev16(registers);
            }

            let addr = registers >> 8;
            let data = registers & 0xFF;
            let mut data_vec = Vec::with_capacity(8);
            let mut addr_vec = Vec::with_capacity(8);
            for i in 0..8 {
                if (data >> i) & 0b1 == 1 {
                    data_vec.push(i)
                }
                if (addr >> i) & 0b1 == 1 {
                    addr_vec.push(i)
                }
            }

            let second_mode = Some(Addr {
                typ: Mode::MultiRegister((addr_vec, data_vec)),
                reg_num: None,
            });



            if dr_bit == 1 {
                Opcode {
                    code,
                    length: length as u32,
                    size: Some(size),
                    src_mode: Some(first_mode),
                    src_value: first_value,
                    src_ext: first_ext,
                    dst_mode: second_mode,
                    dst_value: None,
                    dst_ext: None,
                }
            }
            else {
                Opcode {
                    code,
                    length: length as u32,
                    size: Some(size),
                    src_mode: second_mode,
                    src_value: None,
                    src_ext: None,
                    dst_mode: Some(first_mode),
                    dst_value: first_value,
                    dst_ext: first_ext,
                }
            }

        }
        // TST
        else if high_byte == 0x4A00 {
            let code = Code::Tst;
            let mut length = 2usize;
            let size_bits = (first_word & 0b11000000) >> 6;
            let size = Self::get_size(size_bits);

            let dst_mode_ea = (first_word & 0b111000) >> 3;
            let dst_mode_reg = first_word & 0b111;
            let dst_mode = Self::get_addr_mode(dst_mode_ea, dst_mode_reg);

            let (dst_value, length_inc) = Self::get_value(cn, &dst_mode, pc + length, &size);
            length += length_inc;

            let (dst_ext, length_inc) = Self::get_ext_word(cn, &dst_mode, pc + length);
            length += length_inc;

            Opcode {
                code,
                length: length as u32,
                size: Some(size),
                src_mode: None,
                src_value: None,
                src_ext: None,
                dst_mode: Some(dst_mode),
                dst_value,
                dst_ext,
            }
        }
        // CLR
        else if high_byte == 0x4200 {
            let code = Code::Clr;
            let mut length = 2usize;
            let size_bits = (first_word & 0b11000000) >> 6;
            let size = Self::get_size(size_bits);

            let dst_mode_ea = (first_word & 0b111000) >> 3;
            let dst_mode_reg = first_word & 0b111;
            let dst_mode = Self::get_addr_mode(dst_mode_ea, dst_mode_reg);

            let (dst_value, length_inc) = Self::get_value(cn, &dst_mode, pc + length, &size);
            length += length_inc;

            let (dst_ext, length_inc) = Self::get_ext_word(cn, &dst_mode, pc + length);
            length += length_inc;

            Opcode {
                code,
                length: length as u32,
                size: Some(size),
                src_mode: None,
                src_value: None,
                src_ext: None,
                dst_mode: Some(dst_mode),
                dst_value,
                dst_ext,
            }
        }
        else {
            panic!("Unknown opcode {}", format!("{:b}", first_word));
        }

    }

    fn get_size(bits: u16) -> Size {
        match bits {
            0b00 => Size::Byte,
            0b01 => Size::Word,
            0b10 => Size::Long,
            _ => panic!("Opcode::get_size() size not covered: {}", bits),
        }
    }

    fn get_addr_mode(mode: u16, reg: u16) -> Addr {
        match mode {
            0b000 => Addr { typ: Mode::DataDirect, reg_num: Some(reg) },
            0b001 => Addr { typ: Mode::AddrDirect, reg_num: Some(reg) },
            0b010 => Addr { typ: Mode::AddrIndirect, reg_num: Some(reg) },
            0b011 => Addr { typ: Mode::AddrIndirectPostInc, reg_num: Some(reg) },
            0b100 => Addr { typ: Mode::AddrIndirectPreInc, reg_num: Some(reg) },
            0b101 => Addr { typ: Mode::AddrIndirectDisplace, reg_num: Some(reg) },
            0b110 => Addr { typ: Mode::AddrIndirectIndexDisplace, reg_num: Some(reg) },
            0b111 => match reg {
                0b000 => Addr { typ: Mode::AbsShort, reg_num: None },
                0b001 => Addr { typ: Mode::AbsLong, reg_num: None },
                0b100 => Addr { typ: Mode::Immediate, reg_num: None },
                0b010 => Addr { typ: Mode::PCIndirectDisplace, reg_num: None },
                _ => panic!("Unknown addressing mode {:b} {:b}", mode, reg),
            },
            _ => panic!("Unknown addressing mode {:b} {:b}", mode, reg),
        }
    }

    fn get_value(cn: &Console, mode: &Addr, pos: usize, size: &Size) -> (Option<u32>, usize) {
        let mut length_inc = 0;
        let value = match mode.typ {
            Mode::AbsShort => {
                length_inc += 2;
                Some(cn.rom.read_word(pos) as u32)
            },
            Mode::AbsLong => {
                length_inc += 4;
                Some(cn.rom.read_long(pos))
            },
            Mode::Immediate => {
                match *size {
                    Size::Byte | Size::Word => {
                        length_inc += 2;
                        Some(cn.rom.read_word(pos) as u32)
                    },
                    Size::Long => {
                        length_inc += 4;
                        Some(cn.rom.read_long(pos))
                    },
                }
            },
            _ => None,
        };

        (value, length_inc)
    }

    fn get_ext_word(cn: &Console, mode: &Addr, pos: usize) -> (Option<Ext>, usize) {
        let mut length_inc = 0;
        let value = match mode.typ {
            Mode::AddrIndirectDisplace => {
                length_inc += 2;
                let word = cn.rom.read_word(pos);
                // 2s comple
                let displace = if word >> 15 == 1 {
                    (0x10000 - (word as i64)) * -1
                }
                else {
                    word as i64
                };
                Some(Ext {
                    displace,
                    reg_num: None,
                    reg_type: None,
                    reg_size: None,
                })
            },
            Mode::AddrIndirectIndexDisplace => {
                length_inc += 2;
                let ext_word = cn.rom.read_word(pos) as u32;
                let low_byte = (ext_word & 0xFF) as i64;
                let displace = if low_byte >> 7 == 1 {
                    (0x100 - low_byte) as i64 * -1
                }
                else {
                    low_byte as i64
                };
                let reg_num = (ext_word >> 12) & 0b111;
                let reg_type = (ext_word >> 15) & 1; // 1 == a
                let reg_size = (ext_word >> 11) & 1;

                Some(Ext {
                    displace,
                    reg_num: Some(reg_num),
                    reg_type: Some(if reg_type == 1 {
                        ExtRegType::Addr
                    } else {
                        ExtRegType::Data
                    }),
                    reg_size: Some(if reg_size == 1 {
                        Size::Long
                    } else {
                        Size::Word
                    }),
                })
            },
            Mode::PCIndirectDisplace => {
                length_inc += 2;
                let word = cn.rom.read_word(pos);
                let displace = if word >> 15 == 1 {
                    (0x10000 - (word as i64)) * -1
                }
                else {
                    word as i64
                };
                Some(Ext {
                    displace,
                    reg_num: None,
                    reg_type: None,
                    reg_size: None,
                })
            },
            _ => None,
        };

        (value, length_inc)
    }

    fn get_branch_displacement(cn: &Console, first_word: u16, pos: usize) -> (i64, usize) {
        let mut length_inc = 0;

        let low_byte = first_word & 0xFF;
        let displacement = if low_byte == 0 {
            length_inc += 2;
            let word = cn.rom.read_word(pos);
            // 2s compliment
            if word >> 15 == 1 { // msb
                (0x10000 - (word as i64)) * -1
            }
            else {
                word as i64
            }
        }
        else if low_byte == 0xFF {
            length_inc += 4;
            let long = cn.rom.read_long(pos);
            if long >> 31 == 1 {
                (0x100000000 - (long as i64)) * -1
            }
            else {
                long as i64
            }
        }
        else {
            if low_byte >> 7 == 1 {
                (0x100 - low_byte) as i64 * -1
            }
            else {
                low_byte as i64
            }
        };

        (displacement, length_inc)
    }

    // pretty printing

    pub fn to_string(&self) -> String {
        // initial opcode
        let mut code = format!("\t{}", self.code.to_string().to_lowercase());

        // add size annotation
        match self.size {
            Some(Size::Byte) => code.push_str(".b"),
            Some(Size::Word) => code.push_str(".w"),
            Some(Size::Long) => code.push_str(".l"),
            None => {},
        }

        // check branches
        if self.size.is_none() && self.dst_ext.is_some() {
            code.push_str(match self.length {
                2 => ".s",
                4 => ".w",
                6 => ".l",
                _ => panic!("unknown branch size"),
            });
            code.push_str("\t");
            let displace = self.dst_ext.as_ref().unwrap().displace;
            code.push_str(&format_displace(displace));
        }

        if self.dst_mode.is_some() || self.src_mode.is_some() {
            code.push_str("\t");
        }

        // src
        match self.src_mode {
            None => {},
            Some(ref mode) => {
                let output = match mode.typ {
                    Mode::AbsShort => {
                        format!("(${:X}).w", self.src_value.unwrap())
                    },
                    Mode::AbsLong => {
                        format!("(${:X}).l", self.src_value.unwrap())
                    },
                    Mode::Immediate => {
                        format!("#${:X}", self.src_value.unwrap())
                    },
                    Mode::DataDirect => {
                        format!("d{}", mode.reg_num.unwrap())
                    },
                    Mode::AddrDirect => {
                        format!("a{}", mode.reg_num.unwrap())
                    },
                    Mode::AddrIndirect => {
                        format!("(a{})", mode.reg_num.unwrap())
                    },
                    Mode::AddrIndirectPostInc => {
                        format!("(a{})+", mode.reg_num.unwrap())
                    },
                    Mode::AddrIndirectPreInc => {
                        format!("-(a{})", mode.reg_num.unwrap())
                    },
                    Mode::AddrIndirectDisplace => {
                        let displacement = self.src_ext.as_ref().unwrap().displace;
                        format!("${:X}(a{})", displacement, mode.reg_num.unwrap())
                    },
                    Mode::PCIndirectDisplace => {
                        let displacement = self.src_ext.as_ref().unwrap().displace;
                        format!("{}(pc)", format_displace(displacement))
                    },
                    Mode::AddrIndirectIndexDisplace => {
                        let mode_reg = mode.reg_num.unwrap();
                        let displacement = self.src_ext.as_ref().unwrap().displace;
                        let ext_size = match *self.src_ext.as_ref().unwrap().reg_size.as_ref().unwrap() {
                            Size::Long => ".l",
                            Size::Word => ".w",
                            _ => panic!("this should never happen"),
                        };
                        let ext_reg_type = match *self.src_ext.as_ref().unwrap().reg_type.as_ref().unwrap() {
                            ExtRegType::Data => "d",
                            ExtRegType::Addr => "a",
                        };
                        let ext_reg = self.src_ext.as_ref().unwrap().reg_num.as_ref().unwrap();
                        format!("{}(a{}, {}{}{})",
                            format_displace(displacement),
                            mode_reg,
                            ext_reg_type,
                            ext_reg,
                            ext_size,
                        )
                    },
                    Mode::MultiRegister(ref registers) => {
                        format!("{:?}", registers)
                    },
                };

                code.push_str(&output);
            },
        }


        if self.dst_mode.is_some() && self.src_mode.is_some() {
            code.push_str(", ");
        }

        // dst
        match self.dst_mode {
            None => {},
            Some(ref mode) => {
                let output = match mode.typ {
                    Mode::AbsShort => {
                        format!("(${:X}).w", self.dst_value.unwrap())
                    },
                    Mode::AbsLong => {
                        format!("(${:X}).l", self.dst_value.unwrap())
                    },
                    Mode::DataDirect => {
                        format!("d{}", mode.reg_num.unwrap())
                    },
                    Mode::AddrDirect => {
                        format!("a{}", mode.reg_num.unwrap())
                    },
                    Mode::AddrIndirect => {
                        format!("(a{})", mode.reg_num.unwrap())
                    },
                    Mode::AddrIndirectPostInc => {
                        format!("(a{})+", mode.reg_num.unwrap())
                    },
                    Mode::AddrIndirectPreInc => {
                        format!("-(a{})", mode.reg_num.unwrap())
                    },
                    Mode::AddrIndirectDisplace => {
                        let displacement = self.dst_ext.as_ref().unwrap().displace;
                        format!("{}(a{})", format_displace(displacement), mode.reg_num.unwrap())
                    },
                    Mode::AddrIndirectIndexDisplace => {
                        let mode_reg = mode.reg_num.unwrap();
                        let displacement = self.dst_ext.as_ref().unwrap().displace;
                        let ext_size = match *self.dst_ext.as_ref().unwrap().reg_size.as_ref().unwrap() {
                            Size::Long => ".l",
                            Size::Word => ".w",
                            _ => panic!("this should never happen"),
                        };
                        let ext_reg_type = match *self.dst_ext.as_ref().unwrap().reg_type.as_ref().unwrap() {
                            ExtRegType::Data => "d",
                            ExtRegType::Addr => "a",
                        };
                        let ext_reg = self.dst_ext.as_ref().unwrap().reg_num.as_ref().unwrap();
                        format!("{}(a{}, {}{}{})",
                            format_displace(displacement),
                            mode_reg,
                            ext_reg_type,
                            ext_reg,
                            ext_size,
                        )
                    },
                    Mode::MultiRegister((ref addr, ref data)) => {
                        format!("{:?}", (addr, data))
                    },
                    _ => panic!("Unknown addressing mode (to_string)"),

                };

                code.push_str(&output);
            },
        }


        code
    }
}

fn format_displace(displace: i64) -> String {
    let sign = if displace < 0 {
        "-" } else {
        ""  };
    format!("{}${:X}", sign, displace.abs())
}

// reverse 16 bits
pub fn rev16(x: u16) -> u16 {
   let x = (x & 0x5555) <<  1 | (x & 0xAAAA) >>  1;
   let x = (x & 0x3333) <<  2 | (x & 0xCCCC) >>  2;
   let x = (x & 0x0F0F) <<  4 | (x & 0xF0F0) >>  4;
   let x = (x & 0x00FF) <<  8 | (x & 0xFF00) >>  8;
   x
}
