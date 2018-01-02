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
    pub fn new(path: &str) -> Result<Self, Error> {
        Ok(Console {
            m68k: M68k::new(),
            rom: Rom::new(path)?,
            ram: Ram::new(),
        })
    }

    pub fn start(&mut self) {
        self.m68k.pc = self.rom.entry_point();
        println!("{}", self.m68k);

        let opcode = Opcode::next(&self);
        println!("{}", opcode.to_string());

        println!("{:#?}", opcode);

    }
}
