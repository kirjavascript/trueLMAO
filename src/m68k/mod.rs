#[derive(Debug)]
pub struct M68k {
    pub pc: u32,    // program counter
    cc: u32,
    data: [u16; 8], // data registers
    addr: [u16; 8], // address registers
}

impl M68k {
    pub fn new() -> Self {
        M68k {
            pc: 0,
            cc: 0,
            data: [0; 8],
            addr: [0; 8],
        }
    }
}
