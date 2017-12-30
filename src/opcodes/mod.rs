use console::Console;
use std::fmt;

#[derive(Debug)]
pub struct Opcode {
    code: Code,
    length: usize, // in bytes (amount to increase pc)
    size: Option<Size>, // size of operation
    mode: Option<EAddr>,
    src: Option<u32>,
    dst: Option<u32>,
    // ext: Option<Ext>
}


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

// effective address
#[derive(Debug)]
struct EAddr {
    typ: Mode,
    reg: Option<usize>,
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

macro_rules! basic_opcode {
    ($x:expr) => (
        Opcode {
            code: $x,
            length: 2,
            size: None,
            mode: None,
            src: None,
            dst: None,
        }
    )
}

impl Opcode {
	// clr.b	1(a0,d1.w)
    // pub fn from(cn: Console, addr: usize)

    pub fn next(cn: &Console) -> Self {
        let pc = cn.m68k.pc as usize;
        let next_word = cn.rom.read_word(pc);
        let mut length = 2;

        // NOP
        if next_word == 0x4E71 {
            basic_opcode!(Code::Nop)
        }
        // RTS
        else if next_word == 0x4E75 {
            basic_opcode!(Code::Rts)
        }
        // ILLEGAL
        else if next_word == 0x4AFC {
            basic_opcode!(Code::Illegal)
        }
        // TST
        else if next_word & 0xFF00 == 0x4A00 {
            let code = Code::Tst;
            let size_bits = (next_word & 0b11000000) >> 6;
            let size = Self::get_size(size_bits);
            let mode = Self::get_addr_mode(next_word & 0b111111);

            println!("{}", format!("{:b}", next_word));
            println!("{}", format!("{:x}", next_word));

            let dst = match mode.typ {
                Mode::AbsLong => {
                    length += 4;
                    Some(cn.rom.read_long(pc + 2))
                },
                Mode::AbsShort | Mode::AddrIndirectDisplace => {
                    length += 2;
                    Some(cn.rom.read_word(pc + 2) as u32)
                },
                Mode::Immediate => {
                    None // not supported for TST (on 68000)
                },
                _ => None, // TODO: maybe other modes grab data
            };

            Opcode {
                code,
                length,
                size: Some(size),
                mode: Some(mode),
                src: None,
                dst,
            }
        }
        else {
            panic!("Unknown opcode {}", format!("{:b}", next_word));
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
            0b111000 => EAddr { typ: Mode::AbsShort, reg: None },
            0b111001 => EAddr { typ: Mode::AbsLong, reg: None },
            0b111100 => EAddr { typ: Mode::Immediate, reg: None },
            _ => {
                let reg = Some((bits & 0b111) as usize);
                let mode = bits >> 3;

                match mode {
                    0b000 => EAddr { typ: Mode::DataDirect, reg },
                    0b001 => EAddr { typ: Mode::AddrDirect, reg },
                    0b010 => EAddr { typ: Mode::AddrIndirect, reg },
                    0b011 => EAddr { typ: Mode::AddrIndirectPostInc, reg },
                    0b100 => EAddr { typ: Mode::AddrIndirectPreInc, reg },
                    0b101 => EAddr { typ: Mode::AddrIndirectDisplace, reg },
                    0b110 => EAddr { typ: Mode::AddrIndirectIndexDisplace, reg },
                    _ => panic!("Unknown addressing mode {}", mode),
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
                        code = format!("{}\td{}", code, mode.reg.unwrap());
                    },
                    Mode::AddrDirect => {
                        code = format!("{}\ta{}", code, mode.reg.unwrap());
                    },
                    Mode::AddrIndirect => {
                        code = format!("{}\t(a{})", code, mode.reg.unwrap());
                    },
                    Mode::AddrIndirectPostInc => {
                        code = format!("{}\t(a{})+", code, mode.reg.unwrap());
                    },
                    Mode::AddrIndirectPreInc => {
                        code = format!("{}\t-(a{})", code, mode.reg.unwrap());
                    },
                    Mode::AddrIndirectDisplace => {
                        code = format!("{}\t${:X}(a{})", code, self.dst.unwrap(), mode.reg.unwrap());
                    },
                    Mode::AddrIndirectIndexDisplace => {
                        code = format!("{}\t", code);
                    },
                    _ => panic!("Unknown addressing mode (to_string)"),
                }
            },
        }

        code
    }
}
