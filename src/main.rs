extern crate gtk;

mod console;
mod m68k;
mod ram;
mod rom;
mod opcodes;
mod ui;

use console::Console;
use ui::UI;
use gtk::ButtonExt;

// fn main() {
//     let mut console = Console::new("res/s2.bin").unwrap();
//     let mut console = Console::new("res/test.bin").unwrap();
//     console.start();
//     console.step(); ??
// }
//
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

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let mut console: Rc<RefCell<Console>> = Rc::new(RefCell::new(Console::new("res/test.bin").unwrap()));

    console.borrow_mut().start();

    let mut ui = UI::new(&mut console.borrow_mut());

    let console_clone = console.clone();

    ui.debug_step.connect_clicked(move |_| {
        console_clone.borrow_mut().step();
        // console.step();
    });

    let tick = move || {
        ui.render(&console.borrow_mut());

        gtk::Continue(true)
    };

    gtk::idle_add(tick);
    gtk::main();
}
