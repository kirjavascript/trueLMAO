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

        egui::Window::new("memory")
            .show(ctx, |ui| {

                // emu.core.mem.ram
                let bytes_row = 16;
                let total_bytes = 0x10000;
                let rows = total_bytes / bytes_row;

                egui::ScrollArea::vertical()
                    .max_height(512.)
                    .show_rows(ui, 8., rows, |ui, row_range| {
                        for i in row_range {
                            let offset = i * bytes_row;
                            let bytes = (offset..offset+bytes_row)
                                .enumerate()
                                .map(|(idx, offset)| {
                                    format!(" {:02x}", emu.core.mem.ram[idx + offset])
                                }).collect::<String>();


                            ui.label(format!("{:02x} {}", i, bytes));
                        }
                    });

            });


    }
}
