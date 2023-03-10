pub fn menu<'a>(fullscreen: &'a mut bool, _frame: &'a mut eframe::Frame) ->
    impl egui::Widget + 'a
{
    move |ui: &mut egui::Ui| {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |_ui| {
                #[cfg(not(target_arch = "wasm32"))]
                if _ui.button("Quit").clicked() {
                    _frame.close();
                }
            });

            ui.menu_button("Window", |ui| {
                if ui.button("Auto-arrange").clicked() {
                    ui.ctx().memory_mut(|mem| mem.reset_areas());
                    ui.close_menu();
                }
                if ui.button("Fullscreen").clicked() {
                    *fullscreen = true;
                    ui.close_menu();
                }
            });
        }).response
    }
}
