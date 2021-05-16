use wasm_bindgen::prelude::*;
use lazy_mut::lazy_mut;

use emu::Megadrive;

lazy_mut! {
    static mut EMU: Megadrive = Megadrive::new(
        include_bytes!("/home/cake/sonic/roms/s2.bin").to_vec()
    );
}


#[wasm_bindgen]
pub fn frame() {
    unsafe { EMU.frame(); }
}


#[wasm_bindgen]
pub fn screen() -> *const u8 {
    unsafe { EMU.gfx.screen.as_ptr() }
}
