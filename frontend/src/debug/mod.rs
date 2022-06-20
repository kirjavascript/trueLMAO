pub mod palette;

// pub fn vram(ctx: &egui::Context, ui: &egui::Ui, emu: &mut emu::Megadrive) {
// }
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

}

impl Default for VRAM {
    fn default() -> Self {
        Self {

        }
    }
}

impl VRAM {
    pub fn render(&self, ctx: &egui::Context, emu: &emu::Megadrive) {
        egui::Window::new("vram")
            .vscroll(true)
            .show(ctx, |ui| {
                ui.group(|ui| {
                    ui.label("Within a frame");
                    ui.label("Within a frame");
                });
                // TODO gui palette toggle
                const width: usize = 16;
                const height: usize = 128;
                const pixel_qty: usize = (width * 8) * (height * 8);
                // TODO use retained buffer
                // TODO: widget local state
                let mut pixels: [egui::Color32; pixel_qty] = [ egui::Color32::from_rgb(0, 0, 0); pixel_qty];

                for x_tile in 0..width {
                    for y_tile in 0..height {
                        let offset = x_tile + (y_tile * width);
                        let vram_offset = offset * 32;
                        let mut view_offset = (x_tile * 8) + (y_tile * 8 * (width * 8));

                        for duxel in &emu.core.mem.vdp.VRAM[vram_offset..vram_offset+32] {
                            let pixel = (*duxel & 0xF0) >> 4;

                            let (r, g, b) = emu.core.mem.vdp.cram_rgb[pixel as usize];
                            pixels[view_offset] = egui::Color32::from_rgb(r, g, b);
                            view_offset += 1;
                            let pixel = *duxel & 0xF;
                            let (r, g, b) = emu.core.mem.vdp.cram_rgb[pixel as usize];
                            pixels[view_offset] = egui::Color32::from_rgb(r, g, b);
                            view_offset += 1;
                            if view_offset % 8 == 0 {
                                view_offset += (width-1) * 8;
                            }
                        }

                    }
                }

                let texture: &egui::TextureHandle = &ui.ctx().load_texture(
                    "vram",
                    egui::ColorImage {
                        size: [width*8, height*8],
                        pixels: pixels.to_vec(),
                    },
                    egui::TextureFilter::Nearest
                );
                let img = egui::Image::new(texture, texture.size_vec2() * 2.);

                ui.add(img);
            });
    }
}
