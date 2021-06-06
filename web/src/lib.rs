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
    unsafe { EMU.frame(true); }
}

#[wasm_bindgen]
pub fn skip(quantity: u64) {
    unsafe {
        for _ in 0..quantity {
            EMU.frame(false);
        }
    }
}

#[wasm_bindgen]
pub fn screen() -> *const u8 {
    unsafe { EMU.gfx.screen.as_ptr() }
}

#[wasm_bindgen]
pub fn gamepad_p1(value: usize) {
    unsafe { EMU.core.mem.io.gamepad[0].set(value) }
}

// static mut CANVAS: [u8; 320 * 240 * 4] = [0; 320 * 240 * 4];

// #[wasm_bindgen]
// pub fn canvas() -> *const u8 {
//     let screen = unsafe { EMU.gfx.screen };
//     for (i, pixel) in screen.chunks(3).enumerate() {
//         let index = i * 4;
//         unsafe {
//             CANVAS[index] = pixel[0];
//             CANVAS[index+1] = pixel[1];
//             CANVAS[index+2] = pixel[2];
//             CANVAS[index+3] = 255;
//         }

//     }
//     unsafe { CANVAS.as_ptr() }
// }
