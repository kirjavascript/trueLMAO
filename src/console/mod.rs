use m68k::{M68k,opcode};
use rom::Rom;
use ram::Ram;
use std::io::Error;

#[derive(Debug)]
pub struct Console {
    m68k: M68k,
    rom: Rom,
    ram: Ram,
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

        let next_word = self.rom.read_word(self.m68k.pc as usize);

        println!("{}", self.m68k);
    }
}
