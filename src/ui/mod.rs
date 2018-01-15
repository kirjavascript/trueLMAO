extern crate gtk;

use gtk::prelude::*;
use std::process::exit;
use gtk::Builder;
use gtk::{Window, Button, Label, DrawingArea};
use console::Console;
use std::cell::RefCell;
use std::rc::Rc;

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

        canvas.connect_draw(|_, ctx| {
            ctx.set_dash(&[3., 2., 1.], 1.);
            assert_eq!(ctx.get_dash(), (vec![3., 2., 1.], 1.));

            ctx.scale(500f64, 500f64);

            ctx.set_source_rgb(250.0/255.0, 224.0/255.0, 55.0/255.0);
            ctx.paint();

            ctx.set_line_width(0.05);

            // border
            ctx.set_source_rgb(0.3, 0.3, 0.3);
            ctx.rectangle(0.0, 0.0, 1.0, 1.0);
            ctx.stroke();

            Inhibit(false)
        });

        // let time = "Hello";
        // let label = gtk::Label::new(None);
        // label.set_text(&time);
        // window.add(&label);

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

macro_rules! clone {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move || $body
        }
    );
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move |$(clone!(@param $p),)+| $body
        }
    );
}
