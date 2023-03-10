pub mod palette;
pub mod vram;
pub mod cpu;

use vram::VRAM;

pub struct Debug {
    pub vram: VRAM,
}

impl Default for Debug {
    fn default() -> Self {
        Self {
            vram: Default::default(),
        }
    }
}

impl Debug {
    pub fn render(&mut self, ctx: &egui::Context, emu: &emu::Megadrive) {
        cpu::cpu_window(&ctx, &emu);
        palette::palette_window(&ctx, &emu);
        self.vram.render(&ctx, &emu);



    }
}
