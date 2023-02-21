use wasm_bindgen::prelude::*;
use js_sys::Uint8Array;

use emu::Megadrive;

#[wasm_bindgen]
pub struct MDEmu(Megadrive);

#[wasm_bindgen]
impl MDEmu {
    #[wasm_bindgen(constructor)]
    pub fn new(rom: Uint8Array) -> MDEmu {
        MDEmu(Megadrive::new(rom.to_vec()))
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

    pub fn change_rom(&mut self, rom: Uint8Array) {
        self.0 = Megadrive::new(rom.to_vec())
    }
}
