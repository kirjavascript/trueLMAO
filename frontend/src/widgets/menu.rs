pub fn menu<'a>(fullscreen: &'a mut bool, frame: &'a mut eframe::Frame) ->
    impl egui::Widget + 'a
{
    move |ui: &mut egui::Ui| {
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
                if ui.button("Fullscreen").clicked() {
                    *fullscreen = true;
                    ui.close_menu();
                }
            });
        }).response
    }
}
