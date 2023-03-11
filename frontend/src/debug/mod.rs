pub mod palette;
pub mod vram;
pub mod cpu;

use vram::VRAM;

pub struct Debug {
    pub vram: VRAM,
    tab_index: usize,
}

impl Default for Debug {
    fn default() -> Self {
        Self {
            vram: Default::default(),
            tab_index: 0,
        }
    }
}

const ASCII: &str = r##"................................ !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~................................. ¡¢£¤¥¦§¨©ª«¬­®¯°±²³´µ¶·¸¹º»¼½¾¿ÀÁÂÃÄÅÆÇÈÉÊËÌÍÎÏÐÑÒÓÔÕÖ×ØÙÚÛÜÝÞßàáâãäåæçèéêëìíîïðñòóôõö÷øùúûüýþÿ"##;

impl Debug {
    pub fn render(&mut self, ctx: &egui::Context, emu: &emu::Megadrive) {
        cpu::cpu_window(&ctx, &emu);
        palette::palette_window(&ctx, &emu);
        self.vram.render(&ctx, &emu);


        egui::Window::new("memory")
            .show(ctx, |ui| {

                let tabs: [(&str, usize, Box<dyn Fn(usize) -> u8>); 4] = [
                    ("68K RAM", 0x10000, Box::new(|offset: usize|
                         emu.core.mem.ram[offset])),
                    ("Z80 RAM", 0x2000, Box::new(|offset: usize|
                          emu.core.mem.z80.read_byte(offset as _))),
                    ("ROM", emu.core.mem.rom.size(), Box::new(|offset: usize|
                          emu.core.mem.rom.read_byte(offset as _))),
                    ("IO", 0x20, Box::new(|offset: usize|
                          emu.core.mem.io.read_byte(offset as _))),
                ];

                let (selected_name, total_bytes, accessor) = &tabs[self.tab_index];

                ui.horizontal(|ui| {
                    for (i, (name, _, _)) in tabs.iter().enumerate() {
                        if ui
                            .selectable_label(selected_name == name, *name)
                                .clicked()
                        {
                            self.tab_index = i;
                        }
                    }
                });

                let bytes_row = 16;
                let rows = total_bytes / bytes_row;

                egui::ScrollArea::vertical()
                    .max_height(512.)
                    .show_rows(ui, 8., rows, |ui, row_range| {
                        for i in row_range {
                            let offset = i * bytes_row;
                            let bytes = (offset..offset+bytes_row)
                                .map(|offset| {
                                    format!(" {:02X}", accessor(offset))
                                }).collect::<String>();

                            let ascii = (offset..offset+bytes_row)
                                .map(|offset| {
                                    format!("{}", ASCII.chars().nth(accessor(offset) as _).unwrap_or('.'))
                                }).collect::<String>();
                            ui.monospace(format!("{:06X} {} {}", i * 16, bytes, ascii));
                        }
                    });

            });


    }
}
