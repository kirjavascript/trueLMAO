pub fn palette_window(ctx: &egui::Context, emu: &emu::Megadrive) {
    egui::Window::new("palette")
        .show(ctx, |ui| {
            let pixels = emu.core.mem.vdp.cram_rgb.iter()
                .map(|&(r, g, b)| egui::Color32::from_rgb(r, g, b))
                .collect();
            let texture: &egui::TextureHandle = &ui.ctx().load_texture(
                "palette",
                egui::ColorImage {
                    size: [16, 4],
                    pixels,
                },
                egui::TextureOptions::NEAREST
            );
            let img = egui::Image::new(texture, texture.size_vec2() * 20.);

            ui.add(img);
        });
}
