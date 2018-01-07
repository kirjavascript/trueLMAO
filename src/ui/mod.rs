extern crate gio;
extern crate gtk;

// use gio::prelude::*;
use gtk::prelude::*;
// use std::env::args;
use std::process::exit;

#[derive(Debug)]
pub struct UI {
    label: gtk::Label,
}

impl UI {
    pub fn new() -> Self {
        if gtk::init().is_err() {
            panic!("Failed to initialize GTK.");
        }

        let window = gtk::Window::new(gtk::WindowType::Toplevel);

        window.set_title("trueLMAO");
        window.set_border_width(10);
        window.set_position(gtk::WindowPosition::Center);
        window.set_default_size(640, 480);
        window.set_role("trueLMAO");

        window.connect_delete_event(|_, _| {
            // window.destroy();
            gtk::main_quit();
            exit(0);
            // Inhibit(false)
        });

        let time = "Hello";
        let label = gtk::Label::new(None);
        label.set_text(&time);

        // gtk::events_pending();

        window.add(&label);

        window.show_all();
        let obj = UI {
            label: label,
        };

        obj
    }

    pub fn render(&mut self, i: u32) -> bool {
        // println!("{}", i);
        let num = format!("{}", i);
        self.label.set_text(&num);

        true
    }
}
