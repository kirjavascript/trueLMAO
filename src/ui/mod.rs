use console::Console;
use std::cell::RefCell;
use std::rc::Rc;
use std::process::exit;

use gtk;
use gtk::prelude::*;
use gtk::Builder;
use gtk::{Window, Button, Label, DrawingArea};
use gdk;
use gdk::ContextExt;
use gdk_pixbuf;
use gdk_pixbuf::Pixbuf;

#[derive(Debug)]
pub struct UI {
    debug_cpu: Label,
}

impl UI {
    pub fn new(console: &mut Rc<RefCell<Console>>) -> Self {
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
        let canvas: DrawingArea = builder.get_object("canvas1").unwrap();

        let mut pixbuf: Pixbuf = unsafe {
            Pixbuf::new(0, false, 8, 100, 100).unwrap()
        };

        pixbuf.put_pixel(10, 10, 255, 0, 0, 0);

        canvas.connect_draw(move |_, ctx| {
            // set_source_from_vec ?
            ctx.set_source_pixbuf(&pixbuf, 0f64, 0f64);
            ctx.paint();  // need to call paint() instead of stroke().
            Inhibit(false)
        });

        window.show_all();

        let console_clone = console.clone();

        step.connect_clicked(move |_| {
            console_clone.borrow_mut().step();
        });

        let mut obj = UI {
            debug_cpu: label,
        };



        obj
    }

    pub fn debug_render(&mut self, console: &Console) {
        let num = format!("{}", console.m68k.to_string());
        self.debug_cpu.set_text(&num);
    }
}
