use console::Console;
use std::fmt;

#[derive(Debug)]
pub struct Opcode {
    code: Code,
    pub length: u32, // in bytes (amount to increase pc) ( change to u32 )
    size: Option<Size>, // size of operation
    mode: Option<EAddr>, // effective address
    ext: Option<Ext>, // extension word
    src: Option<u32>,
    dst: Option<u32>,
}

// for move, maybe make src/dst Option<(u32, EAddr)>
// or dst_mode: Option<(EAddr, Ext)>
// get_addr_mode(mode, register)

#[derive(Debug)]
enum Code {
    Tst,
    // Move,
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
struct EAddr {
    typ: Mode,
    reg_num: Option<usize>,
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
            mode: None,
            ext: None,
            src: None,
            dst: None,
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
        // TST
        else if first_word & 0xFF00 == 0x4A00 {
            let code = Code::Tst;
            let mut length = 2;
            let size_bits = (first_word & 0b11000000) >> 6;
            let size = Self::get_size(size_bits);
            let mode = Self::get_addr_mode(first_word & 0b111111);

            let dst = match mode.typ {
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

            let ext = match mode.typ {
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

            // println!("{}", format!("{:0>16b}", ext_word));
            // println!("{:#?}", (displace, reg, reg_type, reg_size));

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
                mode: Some(mode),
                ext,
                src: None,
                dst,
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

    fn get_addr_mode(bits: u16) -> EAddr {
        match bits {
            0b111000 => EAddr { typ: Mode::AbsShort, reg_num: None },
            0b111001 => EAddr { typ: Mode::AbsLong, reg_num: None },
            0b111100 => EAddr { typ: Mode::Immediate, reg_num: None },
            _ => {
                let reg_num = Some((bits & 0b111) as usize);
                let mode = bits >> 3;

                match mode {
                    0b000 => EAddr { typ: Mode::DataDirect, reg_num },
                    0b001 => EAddr { typ: Mode::AddrDirect, reg_num },
                    0b010 => EAddr { typ: Mode::AddrIndirect, reg_num },
                    0b011 => EAddr { typ: Mode::AddrIndirectPostInc, reg_num },
                    0b100 => EAddr { typ: Mode::AddrIndirectPreInc, reg_num },
                    0b101 => EAddr { typ: Mode::AddrIndirectDisplace, reg_num },
                    0b110 => EAddr { typ: Mode::AddrIndirectIndexDisplace, reg_num },
                    _ => panic!("Unknown addressing mode {:b}", mode),
                }
            },
        }
    }

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

        // just dst
        match self.mode {
            None => {},
            Some(ref mode) => {
                match mode.typ {
                    Mode::AbsShort => {
                        code = format!("{}\t(${:X}).w", code, self.dst.unwrap());
                    },
                    Mode::AbsLong => {
                        code = format!("{}\t(${:X}).l", code, self.dst.unwrap());
                    },
                    Mode::DataDirect => {
                        code = format!("{}\td{}", code, mode.reg_num.unwrap());
                    },
                    Mode::AddrDirect => {
                        code = format!("{}\ta{}", code, mode.reg_num.unwrap());
                    },
                    Mode::AddrIndirect => {
                        code = format!("{}\t(a{})", code, mode.reg_num.unwrap());
                    },
                    Mode::AddrIndirectPostInc => {
                        code = format!("{}\t(a{})+", code, mode.reg_num.unwrap());
                    },
                    Mode::AddrIndirectPreInc => {
                        code = format!("{}\t-(a{})", code, mode.reg_num.unwrap());
                    },
                    Mode::AddrIndirectDisplace => {
                        let displacement = self.ext.as_ref().unwrap().displace;
                        code = format!("{}\t${:X}(a{})", code, displacement, mode.reg_num.unwrap());
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
                        code = format!("{}\t${:X}(a{}, {}{}{})",
                            code,
                            displacement,
                            mode_reg,
                            ext_reg_type,
                            ext_reg,
                            ext_size,
                        );
                    },
                    _ => panic!("Unknown addressing mode (to_string)"),
                }
            },
        }

        code
    }
}
