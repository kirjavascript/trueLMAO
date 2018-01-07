extern crate gtk;

mod console;
mod m68k;
mod ram;
mod rom;
mod opcodes;
mod ui;

// use console::Console;
use ui::UI;

// fn main() {
//     let mut console = Console::new("res/s2.bin").unwrap();
//     let mut console = Console::new("res/test.bin").unwrap();
//     console.start();
//     console.step(); ??
// }

fn main() {
    let mut i = 0;

    let mut ui = UI::new();

    let tick = move || {
        i += 1;
        ui.render(i);

        gtk::Continue(true)
    };
    gtk::idle_add(tick);
    gtk::main();
}
