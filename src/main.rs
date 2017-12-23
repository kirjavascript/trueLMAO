mod console;
mod rom;
mod m68k;

use console::Console;

fn main() {
    let mut console = Console::new();

    console.load_rom("s1.bin");
    // console.load_rom("/home/thom/dev/flex2/_test/s2disasm/s2built.bin");
}
