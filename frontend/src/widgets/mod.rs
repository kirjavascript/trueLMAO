pub mod viewport;
pub use viewport::*;

// misc;

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
