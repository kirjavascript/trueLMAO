use rom::Rom;
use m68k::M68k;
use std::io::Error;

#[derive(Debug)]
pub struct Console {
    rom: Rom,
    m68k: M68k,
    // mode
    // region
}

impl Console {
    pub fn new(path: &str) -> Result<Self, Error> {
        Ok(Console {
            rom: Rom::new(path)?,
            m68k: M68k::new(),
        })
    }

    pub fn start(&mut self) {
        self.m68k.pc = self.rom.entry_point();

        println!("{:#?}", self.m68k);
    }
}
