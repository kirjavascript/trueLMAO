extern crate gtk;
extern crate gdk;
extern crate gdk_pixbuf;

mod console;
mod m68k;
mod ram;
mod rom;
mod opcodes;
mod ui;

use console::Console;
use ui::UI;

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let mut console: Rc<RefCell<Console>> = Rc::new(
        RefCell::new(Console::new("res/s2.bin").unwrap())
    );

    console.borrow_mut().start();

    let mut ui = UI::new(&mut console);

    let tick = move || {
        ui.debug_render(&console.borrow());

        gtk::Continue(true)
    };
    gtk::idle_add(tick);
    gtk::main();
}
