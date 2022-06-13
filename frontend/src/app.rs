use emu::Megadrive;
use crate::widgets;
use instant::Instant;
use std::collections::VecDeque;


pub struct Frontend {
    emu: Megadrive,
    fullscreen: bool,
    game_state: GameState,
    test_vec: std::collections::VecDeque<u64>,
}

// TODO: move to core
pub struct GameState {
    pub running: bool,
    pub vsync: bool,
    frames: u64,
    epoch: Instant,
    frames_to_render: u64,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            running: true,
            vsync: true,
            frames: 0,
            frames_to_render: 0,
            epoch: Instant::now(),
        }
    }
}

impl GameState {
    pub fn tick(&mut self) {
        let diff = Instant::now().duration_since(self.epoch);
        let frames = (diff.as_millis() as f64 * 0.05992274) as u64; // TODO: PAL
        // self.emu.gfx.framerate()
        self.frames_to_render = frames - self.frames;
        self.frames = frames;
    }

    pub fn frames_to_render(&self) -> u64 {
        if self.vsync {
            1
        } else {
            self.frames_to_render
        }
    }
}


impl Default for Frontend {
    fn default() -> Self {
        let buf: Vec<u8> = include_bytes!("/home/cake/sonic/roms/s1p.bin").to_vec();
        Self {
            emu: Megadrive::new(buf),
            fullscreen: false,
            game_state: Default::default(),
            test_vec: VecDeque::with_capacity(60),
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

        if self.game_state.running {
            ctx.request_repaint();

            crate::input::dummy_input(ctx, &mut self.emu);

            self.game_state.tick();

            let frames_to_render = self.game_state.frames_to_render();

            if frames_to_render > 3 {
                self.emu.frame(true);
            } else if frames_to_render > 0 {
                for _ in 0..frames_to_render - 1 {
                    self.emu.frame(false);
                }
                self.emu.frame(true);
            }
        }

        // main layout

        if self.fullscreen {
            egui::CentralPanel::default()
                .frame(egui::containers::Frame::none())
                .show(ctx, |ui| {
                    let response = ui.add(widgets::viewport_centred(&self.emu));
                    if response.double_clicked() {
                        self.fullscreen = false;
                    }
                });
            return
        }

        // TODO menu module

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });

                ui.menu_button("Window", |ui| {
                    if ui.button("Auto-arrange").clicked() {
                        ui.ctx().memory().reset_areas();
                        ui.close_menu();
                    }
                });
                // *ui.ctx().memory() = Default::default();
            });
        });

        egui::Window::new("screen")
            .min_height(100.)
            .show(ctx, |ui| {
                let response = ui.add(widgets::viewport(&self.emu));
                if response.double_clicked() {
                    self.fullscreen = true;
                }
            });

        egui::Window::new("palette")
            .show(ctx, |ui| {
                let pixels = self.emu.core.mem.vdp.cram_rgb().iter()
                    .map(|&(r, g, b)| egui::Color32::from_rgb(r, g, b))
                    .collect();
                let texture: &egui::TextureHandle = &ui.ctx().load_texture(
                    "palette",
                    egui::ColorImage {
                        size: [16, 4],
                        pixels,
                    },
                    egui::TextureFilter::Nearest
                );
                let img = egui::Image::new(texture, texture.size_vec2() * 20.);

                ui.add(img);
            });

        egui::CentralPanel::default().show(ctx, |ui| {



            egui::warn_if_debug_build(ui);
            // ctx.inspection_ui(ui);

            ui.label(&format!("MD frames this frame: {}", self.game_state.frames_to_render));
            ui.label(&format!("avg frames {:.1}", self.test_vec.iter().sum::<u64>() as f32 / self.test_vec.len() as f32));

            if ui.button(if self.game_state.running { "pause" } else { "play" }).clicked() {
                self.game_state.running = !self.game_state.running;
            }
            ui.radio_value(&mut self.game_state.vsync, true, "vsync");
            ui.radio_value(&mut self.game_state.vsync, false, "not vsync");

            self.test_vec.push_back(self.game_state.frames_to_render().min(4));

            if self.test_vec.len() > 60 {
                self.test_vec.pop_front();
            }

            use egui::plot::{
                Bar, BarChart, Legend, Plot,
            };
            let chart = BarChart::new(
                self.test_vec
                    .iter()
                    .enumerate()
                    .map(|(i, x)| Bar::new((i + 1) as _, *x as f64))
                    .collect()

            )
            .color(egui::Color32::LIGHT_BLUE)
            .name("Normal Distribution");
            // if !self.vertical {
            //     chart = chart.horizontal();
            // }

            Plot::new("Normal Distribution Demo")
                .width(200.)
                .height(100.)
                .legend(Legend::default())
                .data_aspect(1.0)
                .show(ui, |plot_ui| plot_ui.bar_chart(chart))
                .response
        });

        // TODO debug module

        egui::Window::new("cpu")
            .min_width(800.)
            .show(ctx, |ui| {
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
            });

    }
}
