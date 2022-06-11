use emu::Megadrive;
use crate::widgets;
// use std::time::{Instant, Duration};
use instant::Instant;

pub struct Frontend {
    emu: Megadrive,
    fullscreen: bool,
    game_state: GameState,
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
            vsync: false,
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

            self.game_state.tick();

            let frames_to_render = self.game_state.frames_to_render();

            if frames_to_render > 5 {
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

        egui::Window::new("screen")
            .min_height(100.)
            .show(ctx, |ui| {
                let response = ui.add(widgets::viewport(&self.emu));
                if response.double_clicked() {
                    self.fullscreen = true;
                }
            });


        egui::CentralPanel::default().show(ctx, |ui| {
            egui::warn_if_debug_build(ui);
            // ctx.inspection_ui(ui);
            ui.label(&format!("{:?}", self.game_state.running));
            ui.label(&format!("{:?}", self.game_state.epoch));
            ui.label(&format!("{:?}", self.game_state.frames));
            ui.label(&format!("{:?}", self.game_state.frames_to_render));
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

        // TODO menu module

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
                // TODO: window:arrange / other demo stuff
            });
        });

    }
}
