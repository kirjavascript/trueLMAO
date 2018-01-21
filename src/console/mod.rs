use m68k::M68k;
use rom::Rom;
use ram::Ram;
use opcodes::Opcode;

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
    pub const RES_WIDTH: i32 = 320;
    pub const RES_HEIGHT: i32 = 224;

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
        self.m68k.pc += opcode.length;
        // TODO: cycle counter



        println!("{}", opcode.to_string());
        println!("{:?}", opcode);
    }
}
