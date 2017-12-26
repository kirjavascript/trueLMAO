use console::Console;

#[derive(Debug)]
pub struct Opcode {
    code: Code,
    size: Size,
    // src: Option<u32>,
    // dst: Option<u32>,
    // disasm: String,
}

#[derive(Debug)]
enum Size {
    Byte = 0b00, // TODO: remove c-like
    Word = 0b01,
    Long = 0b10,
}

#[derive(Debug)]
enum Code {
    Tst,
}

impl Opcode {
    pub fn next(cn: &Console) {
        let next_word = cn.rom.read_word(cn.m68k.pc as usize);
        let mut size_bytes = 2;

        // tst
        if next_word & 0xFF00 == 0x4A00 {
            let code = Code::Tst;
            let size_bits = (next_word & 0b1100000000) >> 8;
            let size = Self::get_size(size_bits);
            let mode = Self::get_addr_mode(next_word & 0b111111);


            println!("{}", format!("{:b}", next_word));
        }
        else {
            panic!("Unknown opcode {}", format!("{:b}", next_word));
        }

    }

    fn get_size(bits: u16) -> Size {
        match bits {
            0 => Size::Byte,
            1 => Size::Word,
            2 => Size::Long,
            _ => panic!("Opcode::get_size() size not covered: {}", bits),
        }
    }

    fn get_addr_mode(bits: u16) {

    }

    // pub fn pc_inc()
}
