use r68k_emu::ram::AddressBus;
use r68k_emu::ram::AddressSpace;
use crate::rom::Rom;
use crate::IORef;

use std::cell::RefCell;
use std::rc::Rc;

// file:///home/cake/dev/trueLMAO/target/doc/src/r68k_emu/ram/pagedmem.rs.html#102-140

pub struct Mem {
    io: Rc<RefCell<IO>>,
}

impl Mem {
    pub fn new(io: IORef) -> Self {
        Mem { io }
    }
}

impl AddressBus for Mem {
    fn copy_from(&mut self, other: &Self) {

    }
    fn read_byte(&self, address_space: AddressSpace, address: u32) -> u32 {
        0
    }
    fn read_word(&self, address_space: AddressSpace, address: u32) -> u32 {
        0
    }
    fn read_long(&self, address_space: AddressSpace, address: u32) -> u32 {
        0
    }
    fn write_byte(&mut self, address_space: AddressSpace, address: u32, value: u32) {
    }
    fn write_word(&mut self, address_space: AddressSpace, address: u32, value: u32) {

    }
    fn write_long(&mut self, address_space: AddressSpace, address: u32, value: u32) {

    }
}
