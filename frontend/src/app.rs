use emu::Megadrive;
use crate::widgets;

pub struct Frontend {
    emu:  Megadrive,
    fullscreen: bool,
}

impl Default for Frontend {
    fn default() -> Self {
        let buf: Vec<u8> = include_bytes!("/home/cake/sonic/roms/s1p.bin").to_vec();
        Self {
            emu: Megadrive::new(buf),
            fullscreen: false,
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


        // main layout should go here

        // TODO: self.running
        // TODO: emu_next here * requestrepaint

        if self.fullscreen {
            egui::CentralPanel::default()
                .frame(egui::containers::Frame::none())
                .show(ctx, |ui| {
                    ctx.request_repaint();

                    self.emu.frame(true);

                    let response = ui.add(widgets::viewport(&self.emu.gfx.screen));

                    if response.double_clicked() {
                        self.fullscreen = false;
                    }
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

        // #[cfg(target_arch = "wasm32")]
        // egui::Window::new("file input").show(ctx, |ui| {
        //     use eframe::{wasm_bindgen, web_sys};
        //     use wasm_bindgen::JsCast;

        //     if ui.button("file").clicked() {

        //         let task = rfd::AsyncFileDialog::new().pick_file();

        //         wasm_bindgen_futures::spawn_local(async {
        //             if let Some(file) = task.await {
        //                 web_sys::console::log_1(&format!("{:?}", file.read().await).into());
        //             }
        //         });

        //     }
        // });


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

            self.emu.frame(true);

            let response = ui.add(widgets::viewport(&self.emu.gfx.screen));

            if response.double_clicked() {
                self.fullscreen = true;
            }

        });

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
