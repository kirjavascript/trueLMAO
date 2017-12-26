mod console;
mod m68k;
mod ram;
mod rom;
mod opcodes;

use console::Console;

fn main() {
    let mut console = Console::new("res/s2.bin").unwrap();

    console.start();
}
