pub mod palette;
pub mod vram;
pub mod cpu;
pub mod memory;


pub struct Debug {
    pub vram: vram::VRAM,
    pub memory: memory::Memory,
}

impl Default for Debug {
    fn default() -> Self {
        Self {
            vram: Default::default(),
            memory: Default::default(),
        }
    }
}

impl Debug {
    pub fn render(&mut self, ctx: &egui::Context, emu: &mut emu::Megadrive) {
        cpu::cpu_window(&ctx, &emu);
        palette::palette_window(&ctx, emu);
        self.vram.render(&ctx, &emu);
        self.memory.render(&ctx, &emu);
    }
}
