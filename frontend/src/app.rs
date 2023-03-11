use emu::Megadrive;
use crate::widgets;
use std::collections::VecDeque;

pub struct App {
    emu: Megadrive,
    debug: crate::debug::Debug,
    pub fullscreen: bool,
    pub vsync: bool,
    pub running: bool,
    test_vec: VecDeque<u64>,
}

impl Default for App {
    fn default() -> Self {
        let buf: Vec<u8> = include_bytes!("./s1proto.bin").to_vec();
        Self {
            emu: Megadrive::new(buf),
            debug: Default::default(),
            fullscreen: false,
            vsync: false,
            running: true,
            test_vec: VecDeque::with_capacity(60),
        }
    }
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals {
            dark_mode: true,
            ..egui::Visuals::default()
        });

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        // if let Some(storage) = cc.storage {
        //     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // }


        Default::default()
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

        // game logic

        if self.running {
            ctx.request_repaint();

            crate::input::dummy_input(ctx, &mut self.emu);

            self.emu.render();
        }

        // layout starts with fullscreen

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

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add(crate::widgets::menu(&mut self.fullscreen, frame));
        });

        // game window

        egui::Window::new("screen")
            .min_height(100.)
            .show(ctx, |ui| {
                let response = ui.add(widgets::viewport(&self.emu));
                if response.double_clicked() {
                    self.fullscreen = true;
                }
            });

        // debug stuff

        self.debug.render(&ctx, &self.emu);

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::warn_if_debug_build(ui);
            // ctx.inspection_ui(ui);

            ui.label(&format!("MD frames this frame: {}", self.emu.frame_timer.frame_count));
            ui.label(&format!("avg frames {:.2}", self.test_vec.iter().sum::<u64>() as f32 / self.test_vec.len() as f32));

            if ui.button(if self.running { "pause" } else { "play" }).clicked() {
                self.running = !self.running;
            }
            ui.radio_value(&mut self.vsync, true, "vsync");
            ui.radio_value(&mut self.vsync, false, "not vsync");

            self.test_vec.push_back(self.emu.frame_timer.frame_count.min(4));

            if self.test_vec.len() > 60 {
                self.test_vec.pop_front();
            }
        });

    }
}
