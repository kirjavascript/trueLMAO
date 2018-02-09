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
    Tst, Clr, Jmp, Jsr, Neg,
    Move, Movem,
    And, Sub, Add,
    Cmp,
    Bra, Bsr, Bhi, Bls, Bcc, Bcs, Bne, Beq, Bvc, Bvs, Bpl, Bmi, Bge, Blt, Bgt, Ble,
}

impl fmt::Display for Code {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Code {
    fn is_branch(code: &Code) -> bool {
        match code {
            &Code::Bra | &Code::Bsr | &Code::Bhi | &Code::Bls | &Code::Bcc | &Code::Bcs | &Code::Bne | &Code::Beq | &Code::Bvc | &Code::Bvs | &Code::Bpl | &Code::Bmi | &Code::Bge | &Code::Blt | &Code::Bgt | &Code::Ble => true,
            _ => false,
        }
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
    AddrIndirectIndexDisplace, // (d8, An, Xn)
    PCIndirectDisplace, // (d16, PC)
    PCIndirectIndexDisplace, // (d8, PC, Xn)
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
    fn new_basic(code: Code) -> Opcode {
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

    fn new_branch(code: Code, cn: &Console, first_word: u16, pos: usize) -> Opcode {
        let mut length = 2usize;

        let (displacement, length_inc) = Self::get_branch_displacement(cn, first_word, pos);
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

    pub fn next(cn: &Console) -> Self {
        let pc = cn.m68k.pc as usize;
        Opcode::from(cn, pc)
    }

    pub fn from(cn: &Console, pc: usize) -> Self {
        let first_word = cn.rom.read_word(pc);
        let high_byte = first_word & 0xFF00;
        let high_nybble = first_word & 0xF000;

        // NOP
        if first_word == 0x4E71 {
            Self::new_basic(Code::Nop)
        }
        // RTS
        else if first_word == 0x4E75 {
            Self::new_basic(Code::Rts)
        }
        // ILLEGAL
        else if first_word == 0x4AFC {
            Self::new_basic(Code::Illegal)
        }
        // BRA
        else if high_byte == 0x6000 {
            Self::new_branch(Code::Bra, cn, first_word, pc + 2)
        }
        // BSR
        else if high_byte == 0x6100 {
            Self::new_branch(Code::Bsr, cn, first_word, pc + 2)
        }
        // BHI
        else if high_byte == 0x6200 {
            Self::new_branch(Code::Bhi, cn, first_word, pc + 2)
        }
        // BLS
        else if high_byte == 0x6300 {
            Self::new_branch(Code::Bls, cn, first_word, pc + 2)
        }
        // BCC
        else if high_byte == 0x6400 {
            Self::new_branch(Code::Bcc, cn, first_word, pc + 2)
        }
        // BCS
        else if high_byte == 0x6500 {
            Self::new_branch(Code::Bcs, cn, first_word, pc + 2)
        }
        // BNE
        else if high_byte == 0x6600 {
            Self::new_branch(Code::Bne, cn, first_word, pc + 2)
        }
        // BEQ
        else if high_byte == 0x6700 {
            Self::new_branch(Code::Beq, cn, first_word, pc + 2)
        }
        // BVC
        else if high_byte == 0x6800 {
            Self::new_branch(Code::Bvc, cn, first_word, pc + 2)
        }
        // BVS
        else if high_byte == 0x6900 {
            Self::new_branch(Code::Bvs, cn, first_word, pc + 2)
        }
        // BPL
        else if high_byte == 0x6A00 {
            Self::new_branch(Code::Bpl, cn, first_word, pc + 2)
        }
        // BMI
        else if high_byte == 0x6B00 {
            Self::new_branch(Code::Bmi, cn, first_word, pc + 2)
        }
        // BGE
        else if high_byte == 0x6C00 {
            Self::new_branch(Code::Bge, cn, first_word, pc + 2)
        }
        // BLT
        else if high_byte == 0x6D00 {
            Self::new_branch(Code::Blt, cn, first_word, pc + 2)
        }
        // BGT
        else if high_byte == 0x6E00 {
            Self::new_branch(Code::Bgt, cn, first_word, pc + 2)
        }
        // BLE
        else if high_byte == 0x6F00 {
            Self::new_branch(Code::Ble, cn, first_word, pc + 2)
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
        // AND
        else if high_nybble == 0xC000 {
            let code = Code::And;
            let mut length = 2usize;
            let size_bits = (first_word & 0b11000000) >> 6;
            let size = Self::get_size_normal(size_bits);

            let first_mode_ea = (first_word & 0b111000) >> 3;
            let first_mode_reg = first_word & 0b111;
            let first_mode = Self::get_addr_mode(first_mode_ea, first_mode_reg);

            let (first_value, length_inc) = Self::get_value(cn, &first_mode, pc + length, &size);
            length += length_inc;

            let (first_ext, length_inc) = Self::get_ext_word(cn, &first_mode, pc + length);
            length += length_inc;

            let second_mode = Addr {
                typ: Mode::DataDirect,
                reg_num: Some((first_word & 0b111000000000) >> 9),
            };

            let direction = (first_word & 0b100000000) >> 8;

            if direction == 1 {
                Opcode {
                    code,
                    length: length as u32,
                    size: Some(size),
                    src_mode: Some(second_mode),
                    src_value: None,
                    src_ext: None,
                    dst_mode: Some(first_mode),
                    dst_value: first_value,
                    dst_ext: first_ext,
                }
            }
            else {
                Opcode {
                    code,
                    length: length as u32,
                    size: Some(size),
                    src_mode: Some(first_mode),
                    src_value: first_value,
                    src_ext: first_ext,
                    dst_mode: Some(second_mode),
                    dst_value: None,
                    dst_ext: None,
                }
            }

        }
        // Cmp
        else if high_nybble == 0xB000 {
            let code = Code::Cmp;
            let mut length = 2usize;
            let size_bits = (first_word & 0b11000000) >> 6;
            let size = Self::get_size_normal(size_bits);

            let first_mode_ea = (first_word & 0b111000) >> 3;
            let first_mode_reg = first_word & 0b111;
            let first_mode = Self::get_addr_mode(first_mode_ea, first_mode_reg);

            let (first_value, length_inc) = Self::get_value(cn, &first_mode, pc + length, &size);
            length += length_inc;

            let (first_ext, length_inc) = Self::get_ext_word(cn, &first_mode, pc + length);
            length += length_inc;

            let second_mode = Addr {
                typ: Mode::DataDirect,
                reg_num: Some((first_word & 0b111000000000) >> 9),
            };

            Opcode {
                code,
                length: length as u32,
                size: Some(size),
                src_mode: Some(first_mode),
                src_value: first_value,
                src_ext: first_ext,
                dst_mode: Some(second_mode),
                dst_value: None,
                dst_ext: None,
            }

        }
        // ADD
        else if high_nybble == 0xD000 {
            let code = Code::Add;
            let mut length = 2usize;
            let size_bits = (first_word & 0b11000000) >> 6;
            let size = Self::get_size_normal(size_bits);

            let first_mode_ea = (first_word & 0b111000) >> 3;
            let first_mode_reg = first_word & 0b111;
            let first_mode = Self::get_addr_mode(first_mode_ea, first_mode_reg);

            let (first_value, length_inc) = Self::get_value(cn, &first_mode, pc + length, &size);
            length += length_inc;

            let (first_ext, length_inc) = Self::get_ext_word(cn, &first_mode, pc + length);
            length += length_inc;

            let second_mode = Addr {
                typ: Mode::DataDirect,
                reg_num: Some((first_word & 0b111000000000) >> 9),
            };

            let direction = (first_word & 0b100000000) >> 8;

            if direction == 1 {
                Opcode {
                    code,
                    length: length as u32,
                    size: Some(size),
                    src_mode: Some(second_mode),
                    src_value: None,
                    src_ext: None,
                    dst_mode: Some(first_mode),
                    dst_value: first_value,
                    dst_ext: first_ext,
                }
            }
            else {
                Opcode {
                    code,
                    length: length as u32,
                    size: Some(size),
                    src_mode: Some(first_mode),
                    src_value: first_value,
                    src_ext: first_ext,
                    dst_mode: Some(second_mode),
                    dst_value: None,
                    dst_ext: None,
                }
            }

        }
        // SUB
        else if high_nybble == 0x9000 {
            let code = Code::Sub;
            let mut length = 2usize;
            let size_bits = (first_word & 0b11000000) >> 6;
            let size = Self::get_size_normal(size_bits);

            let first_mode_ea = (first_word & 0b111000) >> 3;
            let first_mode_reg = first_word & 0b111;
            let first_mode = Self::get_addr_mode(first_mode_ea, first_mode_reg);

            let (first_value, length_inc) = Self::get_value(cn, &first_mode, pc + length, &size);
            length += length_inc;

            let (first_ext, length_inc) = Self::get_ext_word(cn, &first_mode, pc + length);
            length += length_inc;

            let second_mode = Addr {
                typ: Mode::DataDirect,
                reg_num: Some((first_word & 0b111000000000) >> 9),
            };

            let direction = (first_word & 0b100000000) >> 8;

            if direction == 1 {
                Opcode {
                    code,
                    length: length as u32,
                    size: Some(size),
                    src_mode: Some(second_mode),
                    src_value: None,
                    src_ext: None,
                    dst_mode: Some(first_mode),
                    dst_value: first_value,
                    dst_ext: first_ext,
                }
            }
            else {
                Opcode {
                    code,
                    length: length as u32,
                    size: Some(size),
                    src_mode: Some(first_mode),
                    src_value: first_value,
                    src_ext: first_ext,
                    dst_mode: Some(second_mode),
                    dst_value: None,
                    dst_ext: None,
                }
            }

        }
        // MOVE
        else if first_word & 0xC000 == 0 {
            let code = Code::Move;
            let mut length = 2usize;
            let size_bits = (first_word & 0b11000000000000) >> 12;
            let size = Self::get_size_move(size_bits);

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
            let size = Self::get_size_normal(size_bit + 1);

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
        // NEG
        else if high_byte == 0x4400 {
            let code = Code::Neg;
            let mut length = 2usize;
            let size_bits = (first_word & 0b11000000) >> 6;
            let size = Self::get_size_normal(size_bits);

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
        // TST
        else if high_byte == 0x4A00 {
            let code = Code::Tst;
            let mut length = 2usize;
            let size_bits = (first_word & 0b11000000) >> 6;
            let size = Self::get_size_normal(size_bits);

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
        // JMP
        else if first_word & 0xFFC0 == 0x4EC0 {
            let code = Code::Jmp;
            let mut length = 2usize;

            let dst_mode_ea = (first_word & 0b111000) >> 3;
            let dst_mode_reg = first_word & 0b111;
            let dst_mode = Self::get_addr_mode(dst_mode_ea, dst_mode_reg);

            let (dst_value, length_inc) = Self::get_value(cn, &dst_mode, pc + length, &Size::Long); // size can be anything... >_>
            length += length_inc;

            let (dst_ext, length_inc) = Self::get_ext_word(cn, &dst_mode, pc + length);
            length += length_inc;

            Opcode {
                code,
                length: length as u32,
                size: None,
                src_mode: None,
                src_value: None,
                src_ext: None,
                dst_mode: Some(dst_mode),
                dst_value,
                dst_ext,
            }
        }
        // JSR
        else if first_word & 0xFFC0 == 0x4E80 {
            let code = Code::Jsr;
            let mut length = 2usize;

            let dst_mode_ea = (first_word & 0b111000) >> 3;
            let dst_mode_reg = first_word & 0b111;
            let dst_mode = Self::get_addr_mode(dst_mode_ea, dst_mode_reg);

            let (dst_value, length_inc) = Self::get_value(cn, &dst_mode, pc + length, &Size::Long); // size can be anything... >_>
            length += length_inc;

            let (dst_ext, length_inc) = Self::get_ext_word(cn, &dst_mode, pc + length);
            length += length_inc;

            Opcode {
                code,
                length: length as u32,
                size: None,
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
            let size = Self::get_size_normal(size_bits);

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

    fn get_size_move(bits: u16) -> Size {
        match bits {
            0b01 => Size::Byte,
            0b11 => Size::Word,
            0b10 => Size::Long,
            _ => panic!("Opcode::get_size_move() size not covered: {}", bits),
        }
    }

    fn get_size_normal(bits: u16) -> Size {
        match bits {
            0b00 => Size::Byte,
            0b01 => Size::Word,
            0b10 => Size::Long,
            _ => panic!("Opcode::get_size_normal() size not covered: {}", bits),
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
                0b011 => Addr { typ: Mode::PCIndirectIndexDisplace, reg_num: None },
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
            Mode::AddrIndirectDisplace | Mode::PCIndirectDisplace => {
                length_inc += 2;
                let word = cn.rom.read_word(pos);
                // 2s comple
                let mut displace = if word >> 15 == 1 {
                    (0x10000 - (word as i64)) * -1
                }
                else {
                    word as i64
                };
                // this could also happen in the cpu instead
                if mode.typ == Mode::PCIndirectDisplace {
                    displace += pos as i64
                }
                Some(Ext {
                    displace,
                    reg_num: None,
                    reg_type: None,
                    reg_size: None,
                })
            },
            Mode::AddrIndirectIndexDisplace | Mode::PCIndirectIndexDisplace => {
                length_inc += 2;
                let ext_word = cn.rom.read_word(pos) as u32;
                let low_byte = (ext_word & 0xFF) as i64;
                let mut displace = if low_byte >> 7 == 1 {
                    (0x100 - low_byte) as i64 * -1
                }
                else {
                    low_byte as i64
                };
                // same here
                if mode.typ == Mode::PCIndirectIndexDisplace {
                    displace += pos as i64
                }
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

        // check branches (better method needed)
        if Code::is_branch(&self.code) {
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
                let output = format_mode(mode, &self.src_value, &self.src_ext);
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
                let output = format_mode(mode, &self.dst_value, &self.dst_ext);
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

fn format_mode(mode: &Addr, value: &Option<u32>, ext: &Option<Ext>) -> String {
    match mode.typ {
        Mode::AbsShort => {
            format!("(${:X}).w", value.unwrap())
        },
        Mode::AbsLong => {
            format!("(${:X}).l", value.unwrap())
        },
        Mode::Immediate => {
            format!("#${:X}", value.unwrap())
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
            let displacement = ext.as_ref().unwrap().displace;
            format!("{}(a{})", format_displace(displacement), mode.reg_num.unwrap())
        },
        Mode::PCIndirectDisplace => {
            let displacement = ext.as_ref().unwrap().displace;
            format!("{}(pc)", format_displace(displacement))
        },
        Mode::AddrIndirectIndexDisplace => {
            format_mode_indexdisplace(mode, &ext)
        },
        Mode::PCIndirectIndexDisplace => {
            format_mode_indexdisplace(mode, &ext)
        },
        Mode::MultiRegister(ref registers) => {
            format!("{:?}", registers)
        },
    }
}

fn format_mode_indexdisplace(mode: &Addr, ext: &Option<Ext>) -> String {
    let base_reg = match mode.reg_num {
        Some(mode_reg) => format!("a{}", mode_reg),
        None => String::from("pc"),
    };
    let displacement = ext.as_ref().unwrap().displace;
    let ext_size = match *ext.as_ref().unwrap().reg_size.as_ref().unwrap() {
        Size::Long => ".l",
        Size::Word => ".w",
        _ => panic!("this should never happen"),
    };
    let ext_reg_type = match *ext.as_ref().unwrap().reg_type.as_ref().unwrap() {
        ExtRegType::Data => "d",
        ExtRegType::Addr => "a",
    };
    let ext_reg = ext.as_ref().unwrap().reg_num.as_ref().unwrap();
    format!("{}({}, {}{}{})",
        format_displace(displacement),
        base_reg,
        ext_reg_type,
        ext_reg,
        ext_size,
    )
}

// reverse 16 bits
pub fn rev16(x: u16) -> u16 {
   let x = (x & 0x5555) <<  1 | (x & 0xAAAA) >>  1;
   let x = (x & 0x3333) <<  2 | (x & 0xCCCC) >>  2;
   let x = (x & 0x0F0F) <<  4 | (x & 0xF0F0) >>  4;
   let x = (x & 0x00FF) <<  8 | (x & 0xFF00) >>  8;
   x
}
