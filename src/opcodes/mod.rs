use console::Console;
use std::fmt;

#[derive(Debug)]
pub struct Opcode {
    code: Code,
    length: usize, // in bytes
    size: Option<Size>, // size of operation
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
enum Size {
    Byte = 0b00, // TODO: remove c-like
    Word = 0b01,
    Long = 0b10,
}

#[derive(Debug)]
enum Mode {
    AbsShort, // (xxx).w
    AbsLong, // (xxx).l
    Immediate, // #<data>*
    // AddrIndirect(u32),
}

impl Opcode {
    pub fn next(cn: &Console) -> Self {
        let next_word = cn.rom.read_word(cn.m68k.pc as usize);
        let mut length = 2;

        // NOP
        if next_word == 0x4E71 {
            Opcode {
                code: Code::Nop,
                size: None,
                length: 2,
            }
        }
        // RTS
        else if next_word == 0x4E75 {
            Opcode {
                code: Code::Rts,
                size: None,
                length: 2,
            }
        }
        // TST
        else if next_word & 0xFF00 == 0x4A00 {
            let code = Code::Tst;
            let size_bits = (next_word & 0b1100000000) >> 8;
            let size = Self::get_size(size_bits);
            let mode = Self::get_addr_mode(next_word & 0b111111);

            println!("{}", format!("{:b}", next_word));

            Opcode {
                code,
                size,
                length,
            }
        }
        else {
            panic!("Unknown opcode {}", format!("{:b}", next_word));
        }

    }

    fn get_size(bits: u16) -> Option<Size> {
        match bits {
            0 => Some(Size::Byte),
            1 => Some(Size::Word),
            2 => Some(Size::Long),
            _ => panic!("Opcode::get_size() size not covered: {}", bits),
        }
    }

    fn get_addr_mode(bits: u16) {
        // match bits {
        //     0b111000 => Mode::AbsShort,
        // }
    }

    pub fn to_string(&self) -> String {
        let mut code = format!("\t{}", self.code.to_string()).to_lowercase();

        code
    }

    // pub fn pc_inc()
    // impl fmt::Display for Opcode
}
