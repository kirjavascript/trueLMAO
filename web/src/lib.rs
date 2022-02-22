use wasm_bindgen::prelude::*;
use lazy_mut::lazy_mut;

use emu::Megadrive;

lazy_mut! {
    static mut EMU: Megadrive = Megadrive::new(
        // include_bytes!("/home/cake/sonic/roms/s2.bin").to_vec()
        include_bytes!("/home/cake/Genesis/Golden Axe II (W) [!].bin").to_vec()
    );
}

#[wasm_bindgen]
pub fn frame(render: bool) {
    unsafe { EMU.frame(render); }
}

#[wasm_bindgen]
pub fn screen() -> *const u8 {
    unsafe { EMU.gfx.screen.as_ptr() }
}

#[wasm_bindgen]
pub fn gamepad_p1(value: usize) {
    unsafe { EMU.core.mem.io.gamepad[0].set(value) }
}
