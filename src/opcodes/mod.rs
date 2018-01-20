use console::Console;
use std::fmt;

#[derive(Debug)]
pub struct Opcode {
    code: Code,
    pub length: u32, // in bytes (amount to increase pc) ( change to u32 )
    size: Option<Size>, // size of operation
    src_mode: Option<Addr>,
    src_value: Option<u32>,
    dst_mode: Option<Addr>,
    dst_value: Option<u32>,
    ext: Option<Ext>, // extension word
}

// parse_addr()
// get_addr_mode(mode, register)

#[derive(Debug)]
enum Code {
    Tst,
    Move,
    Nop, Rts, Illegal,
}

impl fmt::Display for Code {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
enum Size { Byte, Word, Long }

#[derive(Debug)]
struct Addr {
    typ: Mode, // EA
    reg_num: Option<u16>,
}

#[derive(Debug)]
enum Mode {
    DataDirect, // Dn
    AddrDirect, // An
    AddrIndirect, // (An)
    AddrIndirectPostInc, // (An) +
    AddrIndirectPreInc, // - (An)
    AddrIndirectDisplace, // (d16, An)
    AddrIndirectIndexDisplace, // (d8,An,Xn)
    AbsShort, // (xxx).w
    AbsLong, // (xxx).l
    Immediate, // #<data>
}

#[derive(Debug)]
struct Ext {
    displace: u32,
    // the following are only used in IndexDisplacement addressing
    reg_num: Option<u32>,
    reg_type: Option<ExtRegType>,
    reg_size: Option<Size>,
}

#[derive(Debug)]
enum ExtRegType {
    Data, Addr,
}

macro_rules! basic_opcode {
    ($x:expr) => (
        Opcode {
            code: $x,
            length: 2,
            size: None,
            src_mode: None,
            src_value: None,
            dst_mode: None,
            dst_value: None,
            ext: None,
        }
    )
}

impl Opcode {
    pub fn next(cn: &Console) -> Self {
        let pc = cn.m68k.pc as usize;
        Opcode::from(cn, pc)
    }

    pub fn from(cn: &Console, pc: usize) -> Self {
        let first_word = cn.rom.read_word(pc);

        // NOP
        if first_word == 0x4E71 {
            basic_opcode!(Code::Nop)
        }
        // RTS
        else if first_word == 0x4E75 {
            basic_opcode!(Code::Rts)
        }
        // ILLEGAL
        else if first_word == 0x4AFC {
            basic_opcode!(Code::Illegal)
        }
        // MOVE
        else if first_word & 0xC000 == 0 {
            let code = Code::Move;
            let mut length = 2;
            let size_bits = (first_word & 0b11000000) >> 6;
            let size = Self::get_size(size_bits);

            let src_mode_ea = (first_word & 0b111000) >> 3;
            let src_mode_reg = first_word & 0b111;
            let src_mode = Self::get_addr_mode(src_mode_ea, src_mode_reg);

            let dst_mode_reg = (first_word & 0b111000000000) >> 9;
            let dst_mode_ea = (first_word & 0b111000000) >> 6;
            let dst_mode = Self::get_addr_mode(dst_mode_ea, dst_mode_reg);

            println!("{}", format!("{:0>16b}", first_word));

            Opcode {
                code,
                length,
                size: Some(size),
                src_mode: Some(src_mode),
                src_value: None,
                dst_mode: Some(dst_mode),
                dst_value: None,
                ext: None,
            }
        }
        // TST
        else if first_word & 0xFF00 == 0x4A00 {
            let code = Code::Tst;
            let mut length = 2;
            let size_bits = (first_word & 0b11000000) >> 6;
            let size = Self::get_size(size_bits);

            let dst_mode_ea = (first_word & 0b111000) >> 3;
            let dst_mode_reg = first_word & 0b111;
            let dst_mode = Self::get_addr_mode(dst_mode_ea, dst_mode_reg);

            let dst_value = match dst_mode.typ {
                Mode::AbsLong => {
                    length += 4;
                    Some(cn.rom.read_long(pc + 2))
                },
                Mode::AbsShort => {
                    length += 2;
                    Some(cn.rom.read_word(pc + 2) as u32)
                },
                Mode::Immediate => {
                    None // not supported for TST (on 68000)
                },
                _ => None,
            };

            let ext = match dst_mode.typ {
                Mode::AddrIndirectDisplace => {
                    length += 2;
                    Some(Ext {
                        displace: cn.rom.read_word(pc + 2) as u32,
                        reg_num: None,
                        reg_type: None,
                        reg_size: None,
                    })
                },
                Mode::AddrIndirectIndexDisplace => {
                    length += 2;
                    let ext_word = cn.rom.read_word(pc + 2) as u32;
                    let displace = ext_word & 0xFF;
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
                _ => None,
            };

            Opcode {
                code,
                length,
                size: Some(size),
                ext,
                src_mode: None,
                src_value: None,
                dst_mode: Some(dst_mode),
                dst_value,
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
                _ => panic!("Unknown addressing mode {:b} {:b}", mode, reg),
            },
            _ => panic!("Unknown addressing mode {:b} {:b}", mode, reg),
        }
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

        if self.dst_mode.is_some() || self.src_mode.is_some() {
            code.push_str("\t");
        }

        // src
        // ...


        if self.dst_mode.is_some() && self.src_mode.is_some() {
            code.push_str(", ");
        }

        // dst
        let dst = match self.dst_mode {
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
                        let displacement = self.ext.as_ref().unwrap().displace;
                        format!("${:X}(a{})", displacement, mode.reg_num.unwrap())
                    },
                    Mode::AddrIndirectIndexDisplace => {
                        let mode_reg = mode.reg_num.unwrap();
                        let displacement = self.ext.as_ref().unwrap().displace;
                        let ext_size = match *self.ext.as_ref().unwrap().reg_size.as_ref().unwrap() {
                            Size::Long => ".l",
                            Size::Word => ".w",
                            _ => panic!("this should never happen"),
                        };
                        let ext_reg_type = match *self.ext.as_ref().unwrap().reg_type.as_ref().unwrap() {
                            ExtRegType::Data => "d",
                            ExtRegType::Addr => "a",
                        };
                        let ext_reg = self.ext.as_ref().unwrap().reg_num.as_ref().unwrap();
                        format!("${:X}(a{}, {}{}{})",
                            displacement,
                            mode_reg,
                            ext_reg_type,
                            ext_reg,
                            ext_size,
                        )
                    },
                    _ => panic!("Unknown addressing mode (to_string)"),
                };

                code.push_str(&output);
            },
        };


        code
    }
}
