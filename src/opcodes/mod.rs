use console::Console;
use std::fmt;

#[derive(Debug)]
pub struct Opcode {
    code: Code,
    length: usize, // in bytes (amount to increase pc)
    size: Option<Size>, // size of operation
    mode: Option<EAddr>,
    // src: Option<Register>,
    // dst: Option<Register>,
}


#[derive(Debug)]
enum Code {
    Tst,
    Nop,
    Rts,
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
    AddrDirect, // An*
    AddrIndirect, // (An)
    AddrIndirectPostInc, // (An) +
    AddrIndirectPreInc, // - (An)
    AddrIndirectDisplace, // (d16, An)
    // (d8,An,Xn)
    AbsShort, // (xxx).w
    AbsLong, // (xxx).l
    Immediate, // #<data>*
}

impl Opcode {
    pub fn next(cn: &Console) -> Self {
        let next_word = cn.rom.read_word(cn.m68k.pc as usize);
        let mut length = 2;

        // NOP
        if next_word == 0x4E71 {
            Opcode {
                code: Code::Nop,
                length: 2,
                size: None,
                mode: None,
            }
        }
        // RTS
        else if next_word == 0x4E75 {
            Opcode {
                code: Code::Rts,
                length: 2,
                size: None,
                mode: None,
            }
        }
        // TST
        else if next_word & 0xFF00 == 0x4A00 {
            let code = Code::Tst;
            let size_bits = (next_word & 0b11000000) >> 6;
            let size = Self::get_size(size_bits);
            let mode = Self::get_addr_mode(next_word & 0b111111);

            println!("{}", format!("{:b}", next_word));
            println!("{}", format!("{:x}", next_word));
            println!("{:#?}", mode);

            Opcode {
                code,
                length,
                size: Some(size),
                mode: Some(mode),
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
                    0b110 => panic!("(d8,An,Xn) not implemented"),
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

        code
    }

    // impl fmt::Display for Opcode
}
