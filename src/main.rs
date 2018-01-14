extern crate gtk;

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
        RefCell::new(Console::new("res/test.bin").unwrap())
    );

    console.borrow_mut().start();

    let mut ui = UI::new(&mut console);

    let tick = move || {
        ui.render(&console.borrow_mut());

        gtk::Continue(true)
    };

    gtk::idle_add(tick);
    gtk::main();
}
