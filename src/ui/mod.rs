use console::Console;
use std::cell::RefCell;
use std::rc::Rc;
use std::process::exit;

use gtk;
use gtk::prelude::*;
use gtk::Builder;
use gtk::{Window, Button, Label, DrawingArea};
use gdk::ContextExt;
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

        let builder = Builder::new_from_string(include_str!("debug.glade"));

        // debug window
        let debug_window: Window = builder.get_object("debug_window").unwrap();

        debug_window.connect_delete_event(|_, _| {
            // debug_window.destroy();
            gtk::main_quit();
            exit(0);
            // Inhibit(false)
        });

        debug_window.show_all();

        // debug info
        let label = builder.get_object("debug_cpu").unwrap();
        let step: Button = builder.get_object("debug_step").unwrap();

        // render window
        // let render_window: Window = builder.get_object("render_window").unwrap();
        // render_window.show_all();

        // renderer
        // let canvas: DrawingArea = builder.get_object("canvas").unwrap();
        // let pixbuf: Pixbuf = unsafe {
        //     Pixbuf::new(0, false, 8, Console::RES_WIDTH, Console::RES_HEIGHT).unwrap()
        // };
        // pixbuf.put_pixel(0, 0, 255, 0, 0, 0);
        // canvas.connect_draw(move |_, ctx| {
        //     // set_source_from_vec ?
        //     let scale = 2;

        //     let pixbuf_scale: Pixbuf = unsafe {
        //         Pixbuf::new(0, false, 8, Console::RES_WIDTH*scale, Console::RES_HEIGHT*scale).unwrap()
        //     };
        //     pixbuf.scale(
        //         &pixbuf_scale,
        //         0,
        //         0,
        //         Console::RES_WIDTH*scale,
        //         Console::RES_HEIGHT*scale,
        //         0.,
        //         0.,
        //         scale as f64,
        //         scale as f64,
        //         1,
        //     );
        //     ctx.set_source_pixbuf(&pixbuf_scale, 0f64, 0f64);
        //     ctx.paint();
        //     Inhibit(false)
        // });

        let console_clone = console.clone();

        step.connect_clicked(move |_| {
            console_clone.borrow_mut().step();
        });

        UI {
            debug_cpu: label,
        }
    }

    pub fn debug_render(&mut self, console: &Console) {
        let num = format!("{}", console.m68k.to_string());
        self.debug_cpu.set_text(&num);
    }
}
