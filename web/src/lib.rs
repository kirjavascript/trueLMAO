use wasm_bindgen::prelude::*;
use lazy_mut::lazy_mut;

use emu::Megadrive;

lazy_mut! {
    static mut EMU: Megadrive = Megadrive::new();
}

#[wasm_bindgen]
pub fn disasm_stuff() -> String {
    unsafe {
        EMU.disasm_stuff()
    }
}

#[wasm_bindgen]
pub fn step() {
    unsafe {
        EMU.step1();
    }
}
