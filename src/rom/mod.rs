use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::str;

#[derive(Debug)]
pub struct Rom {
    bytes: Vec<u8>,
}

#[allow(dead_code)]
impl Rom {
    pub fn new(path: &str) -> Result<Rom, Error> {
        let f = File::open(path)?;
        let bytes: Vec<u8> = f.bytes().map(|b| b.unwrap()).collect();
        let rom = Rom { bytes };

        // println!("{:?}", format!("{:x}", rom.entry_point()));
        // println!("{:#?}", format!("{:x}", rom.read_byte(rom.entry_point() as usize)));


        Ok(rom)
    }

    // info

    pub fn initial_stack_pointer(&self) -> u32 {
        self.read_long(0)
    }
    pub fn entry_point(&self) -> u32 {
        self.read_long(4)
    }
    pub fn console_name(&self) -> String {
        self.read_string(0x100, 0xF)
    }
    pub fn copyright(&self) -> String {
        self.read_string(0x110, 0xF)
    }
    pub fn domestic(&self) -> String {
        self.read_string(0x120, 0x2F)
    }
    pub fn international(&self) -> String {
        self.read_string(0x150, 0x2F)
    }

    // utils

    fn read_bytes(&self, start: usize, length: usize) -> Vec<u8> {
        let mut tmp = vec![];
        for i in start..(start + length) {
            tmp.push(*self.bytes.get(i).expect("ROM index out of range"));
        }
        tmp
    }

    pub fn read_long(&self, start: usize) -> u32 {
        self.read_bytes(start, 4)
            .iter()
            .fold(0, |acc, c| (acc << 8) + *c as u32)
    }

    pub fn read_word(&self, start: usize) -> u16 {
        self.read_bytes(start, 2)
            .iter()
            .fold(0, |acc, c| (acc << 8) + *c as u16)
    }

    pub fn read_byte(&self, start: usize) -> u8 {
        *self.bytes.get(start).expect("ROM index out of range")
    }

    fn read_string(&self, start: usize, length: usize) -> String {
        let tmp = self.read_bytes(start, length);
        String::from(str::from_utf8(&tmp).unwrap())
    }


    // instructions
}
// https://raw.githubusercontent.com/sonicretro/s2disasm/master/s2.asm
// http://fms.komkon.org/EMUL8/HOWTO.html
//https://emu-docs.org/CPU%2068k/68000inst.txt
//https://github.com/ryanterry131/jaxboy SDL
//
// a0-a7
// d0-d7
// RAM
// VRAM
// GFX
