use rom::Rom;
use m68k::M68k;

#[derive(Debug)]
pub struct Console {
    rom: Option<Rom>,
    m68k: M68k,
    // mode
    // region
}

impl Console {
    pub fn new() -> Self {
        Console {
            rom: None,
            m68k: M68k::new(),
        }
    }

    pub fn load_rom(&mut self, path: &str) {
        self.rom = Rom::new(path).ok(); // convert Result to Option
        self.start();
    }

    pub fn start(&mut self) {
        self.m68k.pc = self.rom.unwrap().entry_point();

        println!("{:#?}", self.m68k);
    }
}
