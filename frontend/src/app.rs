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
        let buf: Vec<u8> = include_bytes!("/home/cake/sonic/roms/s1p.bin").to_vec();
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

        crate::debug::palette::palette_window(&ctx, &self.emu);

        self.debug.vram.render(&ctx, &self.emu);

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

                for (pc, opcode) in emu::debug::disasm_demo(&self.emu) {
                    debug.push_str(&format!("0x{:X}\t{}\n", pc, opcode));
                }
                ui.label(&debug);
            });

    }
}
