pub fn palette_window(ctx: &egui::Context, emu: &mut emu::Megadrive) {
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

            // let rgb = &emu.core.mem.vdp.cram_rgb;

            // for i in 0..rgb.len() {

            //     let (r, g, b) = emu.core.mem.vdp.cram_rgb[i];
            //     let mut color = [r, g, b];
            //     ui.color_edit_button_srgb(&mut color);
            //     emu.core.mem.vdp.cram_rgb[i] = (color[0], color[1], color[2]);
            // }
        });

}
