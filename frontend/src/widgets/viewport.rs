use emu::Megadrive;

fn viewport_ui(ui: &mut egui::Ui, emu: &Megadrive, centered: bool) -> egui::Response {
    let pixels = emu.gfx.screen.chunks_exact(3)
        .map(|p| egui::Color32::from_rgb(p[0], p[1], p[2]))
        .collect();
    let texture: &egui::TextureHandle = &ui.ctx().load_texture(
        "viewport",
        egui::ColorImage {
            size: [320, 240],
            pixels,
        },
        egui::TextureFilter::Nearest
    );

    let (width, height) = (ui.available_width(), ui.available_height());

    let size = texture.size_vec2();

    let size_w = size * (width / size.x);

    let size = if size_w.y > height {
        size * (height / size.y)
    } else {
        size_w
    };

    let img = egui::Image::new(texture, size)
        .sense(egui::Sense::click());

    if centered {
        let y_margin = (height - size.y) / 2.;
        let x_margin = (width - size.x) / 2.;

        let (r, g, b) = emu.core.mem.vdp.bg_color();

        egui::Frame::none()
            .fill(egui::Color32::from_rgb(r, g, b))
            // .sense(egui::Sense::click())
            .inner_margin(egui::style::Margin::symmetric(x_margin, y_margin))
            .show(ui, |ui| {
                ui.add(img)


            })
            .inner
            .union(ui.interact(
                egui::Rect::EVERYTHING,
                ui.id(),
                egui::Sense::click()
            ))
    } else {
        ui.add(img)
    }
}

pub fn viewport(emu: &Megadrive) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| viewport_ui(ui, emu, false)
}

pub fn viewport_centred(emu: &Megadrive) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| viewport_ui(ui, emu, true)
}
