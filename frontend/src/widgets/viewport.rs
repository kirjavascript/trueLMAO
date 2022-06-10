use emu::gfx::Screen;

fn viewport_ui(ui: &mut egui::Ui, screen: &Screen) -> egui::Response {
    let pixels = screen.chunks_exact(3)
        .map(|p| egui::Color32::from_rgba_unmultiplied(p[0], p[1], p[2], 255))
        .collect();
    let texture: &egui::TextureHandle = &ui.ctx().load_texture(
        "viewport",
        egui::ColorImage {
            size: [320, 240],
            pixels,
        },
        egui::TextureFilter::Nearest
    );
    let img_size = ui.available_height() * texture.size_vec2() / texture.size_vec2().y;

    let img = egui::Image::new(texture, img_size);

    ui.add(img.sense(egui::Sense::click()))
}

pub fn viewport(screen: &Screen) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| viewport_ui(ui, screen)
}
