mod app;
mod debug;
mod input;
mod widgets;
pub use app::App;


#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::{self, prelude::*};

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn start() {
    // make sure panics are logged using `console.error`
    console_error_panic_hook::set_once();

    // redirect tracing to console.log and friends
    tracing_wasm::set_as_global_default();

    eframe::start_web(
        "emu",
        Default::default(),
        Box::new(|cc| Box::new(App::new(cc)))
    ).expect("eframe didnt load");
}
