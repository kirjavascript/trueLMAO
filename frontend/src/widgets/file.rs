type File = Vec<u8>;

// wasm

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use web_sys::{window, console, Element, HtmlInputElement, FileReader};
#[cfg(target_arch = "wasm32")]
use js_sys::{Uint8Array, ArrayBuffer, Object};


#[cfg(target_arch = "wasm32")]
pub struct FileDialog {
    tx: std::sync::mpsc::Sender<File>,
    rx: std::sync::mpsc::Receiver<File>,
    input: HtmlInputElement,
    closure: Option<Closure<dyn FnMut()>>,
}

#[cfg(target_arch = "wasm32")]
impl Default for FileDialog {
    fn default() -> Self {
        let (tx, rx) = std::sync::mpsc::channel();

        let document = window().unwrap().document().unwrap();
        let body = document.body().unwrap();
        let input = document.create_element("input").unwrap().dyn_into::<HtmlInputElement>().unwrap();
        input.set_attribute("type", "file").unwrap();
        input.style().set_property("display", "none").unwrap();
        body.append_child(&input).unwrap();

        Self {
            rx,
            tx,
            input,
            closure: None,
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl Drop for FileDialog {
    fn drop(&mut self) {
        self.input.remove();
        if self.closure.is_some() {
            std::mem::replace(&mut self.closure, None).unwrap().forget();
        }
    }
}

#[cfg(target_arch = "wasm32")]
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
                    tx.send(buffer);
                }));

                reader.set_onload(Some(onload_closure.as_ref().unchecked_ref()));
                reader.read_as_array_buffer(&file).unwrap();
                onload_closure.forget();
            }
        });

        self.input.add_event_listener_with_callback("change", closure.as_ref().unchecked_ref()).unwrap();
        self.closure = Some(closure);
        self.input.click();
    }

    pub fn opened(&self) -> Option<Vec<u8>> {
        if let Ok(file) = self.rx.try_recv() {
            Some(file)
        } else {
            None
        }
    }

}

// native

#[cfg(not(target_arch = "wasm32"))]
use rfd;

#[cfg(not(target_arch = "wasm32"))]
pub struct FileDialog {
    file: Option<File>,
}

#[cfg(not(target_arch = "wasm32"))]
impl Default for FileDialog {
    fn default() -> Self {
        Self {
            file: None,
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl FileDialog {
    pub fn open_file(&mut self) {
        let path = rfd::FileDialog::new().pick_file();
        if let Some(path) = path {
            self.file = std::fs::read(path).ok();
        }
    }

    pub fn opened(&mut self) -> Option<File> {
        std::mem::replace(&mut self.file, None)
    }

}
