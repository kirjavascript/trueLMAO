extern crate gtk;

use gtk::prelude::*;
use std::process::exit;
use gtk::Builder;
use gtk::{Window, Button, Label};
use console::Console;

#[derive(Debug)]
pub struct UI {
    debug_cpu: Label,
    pub debug_step: Button,
}

impl UI {
    pub fn new(console: &mut Console) -> Self {
        if gtk::init().is_err() {
            panic!("Failed to initialize GTK.");
        }

        let glade_src = include_str!("debug.glade");
        let builder = Builder::new_from_string(glade_src);
        let window: Window = builder.get_object("debug_window").unwrap();
        window.show_all();

        window.connect_delete_event(|_, _| {
            // window.destroy();
            gtk::main_quit();
            exit(0);
            // Inhibit(false)
        });

        let label = builder.get_object("label1").unwrap();
        let step: Button = builder.get_object("debug_step").unwrap();

        // let time = "Hello";
        // let label = gtk::Label::new(None);
        // label.set_text(&time);
        // window.add(&label);

        window.show_all();

        // step.connect_clicked(|_| {
        //     println!("boo");
        // });

        let mut obj = UI {
            debug_cpu: label,
            debug_step: step,
        };



        obj
    }

    pub fn render(&mut self, console: &Console) {
        let num = format!("{}", console.m68k.to_string());
        self.debug_cpu.set_text(&num);
    }
}
