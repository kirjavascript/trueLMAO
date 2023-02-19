use wasm_bindgen::prelude::*;

use emu::Megadrive;

#[wasm_bindgen]
pub struct MDEmu(Megadrive);

#[wasm_bindgen]
impl MDEmu {
    #[wasm_bindgen(constructor)]
    pub fn new() -> MDEmu {
        MDEmu(Megadrive::new(
            include_bytes!("/home/cake/sonic/roms/s1p.bin").to_vec()
        ))
    }


    pub fn render(&mut self) -> u64  {
        self.0.render()
    }

    pub fn screen(&self) -> *const u8 {
        self.0.gfx.screen.as_ptr()
    }

    pub fn gamepad_p1(&mut self, value: usize) {
        self.0.core.mem.io.gamepad[0].set(value)
    }
}
