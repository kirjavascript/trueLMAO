pub mod palette;

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

pub struct VRAM {
    palette_line: usize,
}

impl Default for VRAM {
    fn default() -> Self {
        Self {
            palette_line: 0,
        }
    }
}

impl VRAM {
    pub fn render(&mut self, ctx: &egui::Context, emu: &emu::Megadrive) {
        egui::Window::new("vram")
            .vscroll(true)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.radio_value(&mut self.palette_line, 0, "0");
                    ui.radio_value(&mut self.palette_line, 1, "1");
                    ui.radio_value(&mut self.palette_line, 2, "2");
                    ui.radio_value(&mut self.palette_line, 3, "3");
                });

                const WIDTH: usize = 16;
                const HEIGHT: usize = 128;
                const PIXEL_QTY: usize = (WIDTH * 8) * (HEIGHT * 8);
                // TODO use retained buffer
                // TODO: only show onscreen ram"
                let mut pixels: [egui::Color32; PIXEL_QTY] = [ egui::Color32::from_rgb(0, 0, 0); PIXEL_QTY];

                let palette_offset = self.palette_line * 0x10;
                for x_tile in 0..WIDTH {
                    for y_tile in 0..HEIGHT {
                        let offset = x_tile + (y_tile * WIDTH);
                        let vram_offset = offset * 32;
                        let mut view_offset = (x_tile * 8) + (y_tile * 8 * (WIDTH * 8));

                        for duxel in &emu.core.mem.vdp.VRAM[vram_offset..vram_offset+32] {
                            let pixel = (*duxel & 0xF0) >> 4;

                            let (r, g, b) = emu.core.mem.vdp.cram_rgb[palette_offset + pixel as usize];
                            pixels[view_offset] = egui::Color32::from_rgb(r, g, b);
                            view_offset += 1;
                            let pixel = *duxel & 0xF;
                            let (r, g, b) = emu.core.mem.vdp.cram_rgb[palette_offset + pixel as usize];
                            pixels[view_offset] = egui::Color32::from_rgb(r, g, b);
                            view_offset += 1;
                            if view_offset % 8 == 0 {
                                view_offset += (WIDTH-1) * 8;
                            }
                        }

                    }
                }

                let texture: &egui::TextureHandle = &ui.ctx().load_texture(
                    "vram",
                    egui::ColorImage {
                        size: [WIDTH*8, HEIGHT*8],
                        pixels: pixels.to_vec(),
                    },
                    egui::TextureFilter::Nearest
                );
                let img = egui::Image::new(texture, texture.size_vec2() * 2.);

                ui.add(img);
            });
    }
}
