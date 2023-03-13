// use std::future::Future;
// use eframe::{egui, epi};
// use rfd;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, console, Element, HtmlInputElement, FileReader};
use js_sys::{Uint8Array, ArrayBuffer, Object};

pub enum Message {
    File(Vec<u8>),
    // Other messages
}

pub struct FileDialog {
    tx: std::sync::mpsc::Sender<Message>,
    rx: std::sync::mpsc::Receiver<Message>,
    input: HtmlInputElement,
    closure: Option<Closure<dyn FnMut()>>,
}

// #[cfg(not(target_arch = "wasm32"))]

impl Default for FileDialog {
    #[cfg(target_arch = "wasm32")]
    fn default() -> Self {
        let (tx, rx) = std::sync::mpsc::channel();

        let document = window().unwrap().document().unwrap();
        let body = document.body().unwrap();
        let input = document.create_element("input").unwrap().dyn_into::<HtmlInputElement>().unwrap();
        input.set_attribute("type", "file").unwrap();
        input.style().set_property("display", "none").unwrap();
        body.append_child(&input).unwrap();


        // let input_clone = input.clone();
        // let closure = Closure::wrap(Box::new(move || {
        //     if let Some(file) = input_clone.files().and_then(|files| files.get(0)) {
        //         let reader = FileReader::new().unwrap();
        //         let reader_clone = reader.clone();
        //         let onload_closure = Closure::wrap(Box::new(move || {
        //             let array_buffer = reader_clone.result().unwrap().dyn_into::<ArrayBuffer>().unwrap();
        //             let buffer = Uint8Array::new(&array_buffer).to_vec();
        //             console::log_1(&format!("File data buffer: {:?}", buffer).into());
        //         }) as Box<dyn FnMut()>);

        //         reader.set_onload(Some(onload_closure.as_ref().unchecked_ref()));
        //         reader.read_as_array_buffer(&file).unwrap();
        //         onload_closure.forget();
        //     }
        // }) as Box<dyn FnMut()>);
        // input.add_event_listener_with_callback("change", closure.as_ref().unchecked_ref()).unwrap();
        // closure.forget();

        Self {
            rx,
            tx,
            input,
            closure: None,
        }
    }
}

impl Drop for FileDialog {
    fn drop(&mut self) {
        self.input.remove();
    }
}

impl FileDialog {
    pub fn open_file(&mut self) {
        if let Some(closure) = &self.closure {
            self.input.remove_event_listener_with_callback("change", closure.as_ref().unchecked_ref()).unwrap();
            std::mem::replace(&mut self.closure, None).unwrap().forget();
        }

        let tx = self.tx.clone();
        let input_clone = self.input.clone();

        let closure = Closure::once(move || {
            if let Some(file) = input_clone.files().and_then(|files| files.get(0)) {
                let reader = FileReader::new().unwrap();
                let reader_clone = reader.clone();
                let onload_closure = Closure::once(Box::new(move || {
                    let array_buffer = reader_clone.result().unwrap().dyn_into::<ArrayBuffer>().unwrap();
                    let buffer = Uint8Array::new(&array_buffer).to_vec();
                    console::log_1(&format!("File data buffer: {:?}", buffer).into());
                    tx.send(Message::File(buffer));
                }));

                reader.set_onload(Some(onload_closure.as_ref().unchecked_ref()));
                reader.read_as_array_buffer(&file).unwrap();
                onload_closure.forget();
            }
        });

        self.input.add_event_listener_with_callback("change", closure.as_ref().unchecked_ref()).unwrap();
        self.closure = Some(closure);
        self.input.click();



            // if ui.button("Open file…").clicked() {
            //     if let Some(path) = rfd::AsyncFileDialog::new().pick_file() {
            //         let _ = Some(path.display().to_string());
            //     }
            // }
    }
}

// pub fn filedialog(state: FileState) ->
//     impl egui::Widget
// {
//     move |ui: &mut egui::Ui| {

// }


// impl epi::App for FileApp {
//     fn name(&self) -> &str {
//         "file dialog app"
//     }
//     fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
//         // This is important, otherwise file dialog can hang
//         // and messages are not processed
//         ctx.request_repaint();

//         loop {
//             match self.message_channel.1.try_recv() {
//                 Ok(_message) => {
//                     // Process FileOpen and other messages
//                 }
//                 Err(_) => {
//                     break;
//                 }
//             }
//         }

//         egui::CentralPanel::default().show(ctx, |ui| {
//             let open_button = ui.add(egui::widgets::Button::new("Open..."));

//             if open_button.clicked() {
//                 let task = rfd::AsyncFileDialog::new()
//                     .add_filter("Text files", &["txt"])
//                     .set_directory("/")
//                     .pick_file();

//                 let message_sender = self.message_channel.0.clone();

//                 execute(async move {
//                     let file = task.await;

//                     if let Some(file) = file {
//                         message_sender.send(file.read().await).ok();
//                     }
//                 });

//             }
//         });
//     }
// }

// #[cfg(not(target_arch = "wasm32"))]
// fn execute<F: Future<Output = ()> + Send + 'static>(f: F) {
//     std::thread::spawn(move || futures::executor::block_on(f));
// }
// #[cfg(target_arch = "wasm32")]
// fn execute<F: Future<Output = ()> + 'static>(f: F) {
//     wasm_bindgen_futures::spawn_local(f);
// }
