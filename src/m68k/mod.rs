use std::fmt;

#[derive(Debug)]
pub struct M68k {
    pub pc: u32,    // program counter
    cc: u8,         // ________-___XNZVC & 0b11111)
    data: [u32; 8], // data registers (longword)
    addr: [u32; 8], // address registers (longword)
}

// status register  - X-Extend, N-Negative, Z-Zero, V-Overlow, C-Carry
// a7 - active stack pointer
// operations of registers affect the lower bits

impl fmt::Display for M68k {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "PC: 0x{:x}", self.pc)?;
        writeln!(f, "data: {:?}", self.data)?;
        writeln!(f, "addr: {:?}", self.addr)?;
        write!(f, "flags: 0b{:0>5b} (XNZVC)", self.cc)
    }
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

    pub fn setZ(&mut self) {
        self.cc = self.cc & 0b00100;
    }
}
