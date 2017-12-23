mod console;
mod rom;
mod m68k;

use console::Console;

fn main() {
    let mut console = Console::new("res/s2.bin").unwrap();

    console.start();
}
