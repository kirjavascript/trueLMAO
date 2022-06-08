use emu::Megadrive;

pub struct Frontend {
    emu:  Megadrive,
    fullscreen: bool,
    tmp_zoom: f32,
}

impl Default for Frontend {
    fn default() -> Self {
        let buf: Vec<u8> = include_bytes!("/home/cake/sonic/roms/s1p.bin").to_vec();
        Self {
            emu: Megadrive::new(buf),
            fullscreen: false,
            tmp_zoom: 400.0,
        }
    }
}

impl Frontend {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals {
            dark_mode: true,
            ..egui::Visuals::default()
        });

        // let mut fonts = egui::FontDefinitions::default();

        // fonts
        //     .families
        //     .entry(egui::FontFamily::Monospace)
        //     .or_default()
        //     .push("Hack".to_string());

        // cc.egui_ctx.set_fonts(fonts);

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        // if let Some(storage) = cc.storage {
        //     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // }


        Default::default()
    }
}

impl eframe::App for Frontend {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

        if self.fullscreen {
            egui::CentralPanel::default().show(ctx, |ui| {
                ctx.request_repaint();

                let response = ui.interact(
                    egui::Rect::EVERYTHING,
                    ui.id(),
                    egui::Sense::click()
                );
                if response.double_clicked() {
                    self.fullscreen = false;
                }

                self.emu.frame(true);
                let pixels = self.emu.gfx.screen.chunks_exact(3)
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

                // let mut size = egui::Vec2::new(image.size[0] as f32, image.size[1] as f32);
                // size *= (ui.available_width() / size.x).min(1.0);
                // ui.image(texture_id, size);

                ui.image(texture, img_size);
            });

            return

        }
            // ui.heading("Side Panel");

            // let mut value: f32 = 1.;

            // ui.horizontal(|ui| {
            //     ui.label("Write something: ");
            //     ui.text_edit_singleline(&mut "asd");
            // });

            // ui.add(egui::Slider::new(&mut value, 0.0..=10.0).text("value"));
            // if ui.button("Increment").clicked() {
            //     value += 1.0;
            // }

        #[cfg(target_arch = "wasm32")]
        egui::Window::new("file input").show(ctx, |ui| {
            use eframe::{wasm_bindgen, web_sys};
            use wasm_bindgen::JsCast;
            let text_agent: web_sys::HtmlInputElement = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .get_element_by_id("egui_text_agent")
                .unwrap()
                .dyn_into()
                .unwrap();

            text_agent.set_type("file");

            // file_agent / widget

            if ui.button("file").clicked() {
                text_agent.click();

            }
        });


        egui::Window::new("filter").show(ctx, |ui| {

            let texture: &egui::TextureHandle = &ui.ctx().load_texture(
                "test",
                egui::ColorImage {
                    size: [2, 2],
                    pixels: vec![
                        egui::Color32::from_rgb(255, 255, 0),
                        egui::Color32::from_rgb(255, 0, 0),
                        egui::Color32::from_rgb(0, 255, 0),
                        egui::Color32::from_rgb(0, 0, 255),
                    ],
                },
                egui::TextureFilter::Nearest
            );
            let img_size = 20. * texture.size_vec2() / texture.size_vec2().y;
            ui.image(texture, img_size);

        });


        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::Window::new("screen").show(ctx, |ui| {
            ctx.request_repaint();

            ui.add(egui::Slider::new(&mut self.tmp_zoom, 100.0..=700.0).text("tmp zoom"));

            let response = ui.interact(
                egui::Rect::EVERYTHING,
                ui.id(),
                egui::Sense::click()
            );
            if response.double_clicked() {
                self.fullscreen = true;
            }


            self.emu.frame(true);
            let pixels = self.emu.gfx.screen.chunks_exact(3)
                .map(|p| egui::Color32::from_rgb(p[0], p[1], p[2]))
                .collect();
            let texture: &egui::TextureHandle = &ui.ctx().load_texture(
                "viewport",
                egui::ColorImage {
                    size: [320, 240],
                    pixels,
                },
                // TODO: blurryness https://github.com/emilk/egui/pull/1636
                egui::TextureFilter::Nearest
            );
            let img_size = self.tmp_zoom * texture.size_vec2() / texture.size_vec2().y;
            ui.image(texture, img_size);

        });

        // egui::Window::new("hq3x").show(ctx, |ui| {
        //     ctx.request_repaint();


        //     let mut pixels3: [u8; 320*240*4] = [0;320*240*4];
        //     let mut pixels2: [egui::Color32; 320*240] = [egui::Color32::DARK_RED;320*240];
        //     static mut pixels32: [u32; 320*240] = [0;320*240];
        //     let mut pixels4: [u32; 320*240*3*3] = [0;320*240*3*3];
        //     let mut pixels42: [egui::Color32; 320*240*3*3] = [egui::Color32::DARK_RED;320*240*3*3];

        //     self.emu.gfx.screen.chunks_exact(3)
        //         .enumerate()
        //         .for_each(|(i, p)| {
        //             let index = i * 4;
        //             pixels3[index] = p[0];
        //             pixels3[index+1] = p[1];
        //             pixels3[index+2] = p[2];
        //             pixels3[index+3] = 255;
        //         });

        //     pixels3.chunks_exact(4)
        //         .enumerate()
        //         .for_each(|(i, p)| {
        //             unsafe {
        //                 pixels32[i] = ((p[0]as u32) << 24) + ((p[1]as u32) <<16) + ((p[2]as u32)<<8 ) + 255;
        //             }
        //         });

        //     unsafe {

        //         hqx::hq3x(&pixels32, &mut pixels4, 320, 240);
        //     }

        //     for (i, foo) in pixels4.iter().enumerate() {
        //         pixels42[i] = egui::Color32::from_rgb(
        //             (foo >> 24) as u8,
        //             (foo >> 16) as u8 & 0xFF,
        //             (foo >> 8) as u8 & 0xFF,
        //         );
        //     }
        //     let texture: &egui::TextureHandle = &ui.ctx().load_texture(
        //         "viewport",
        //         egui::ColorImage {
        //             size: [320*3, 240*3 ],
        //             pixels: pixels42.to_vec(),
        //         },
        //         egui::TextureFilter::Nearest
        //     );
        //     let img_size = self.tmp_zoom * texture.size_vec2() / texture.size_vec2().y;
        //     ui.image(texture, img_size);

        // });

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::warn_if_debug_build(ui);

            let mut debug = String::new();
            debug.push_str(&format!("PC: {:X}\n\n", self.emu.core.pc));


            // let v = self.emu.core.mem.vdp.VSRAM.iter().map(|x|format!("{:X}", x)).collect::<Vec<String>>().join(" ");
            // debug.push_str(&format!("VSRAM: {}\n\n", v));

            debug.push_str(&format!("D "));
            for i in 0..=7 {
                debug.push_str(&format!("{:X} ", self.emu.core.dar[i]));
            }
            debug.push_str(&format!("\n"));

            debug.push_str(&format!("A "));
            for i in 0..=7 {
                debug.push_str(&format!("{:X} ", self.emu.core.dar[i + 8]));
            }
            debug.push_str(&format!("\n"));
            debug.push_str(&format!("\n"));

            for (pc, opcode) in self.emu.disasm() {
                debug.push_str(&format!("0x{:X}\t{}\n", pc, opcode));
            }
            ui.label(&debug);
            // ctx.inspection_ui(ui);
        });

    }
}
