use wasm_bindgen::prelude::*;
use lazy_static::lazy_static;

use emu::Emulator;
// use std::cell::RefCell;
// lazy_static! {
//     static ref EMU: RefCell<Emulator> = RefCell::new(Emulator::new());
// }
static mut EMU: Emulator = Emulator::new();

#[wasm_bindgen]
pub fn new_system() -> & mut Emulator {
    let emu = Box::new(EMU);
    Box::leak(emu)
}
