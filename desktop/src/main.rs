use fltk::{
    app,
    button::Button,
    frame::Frame,
    prelude::*,
    window::Window,
    table::Table,
    text::{TextBuffer, TextDisplay},
    dialog,
};

use emu::Emulator;

#[derive(Debug, Copy, Clone)]
pub enum Update {
    Render, Step,
}

fn main() {
    let app = app::App::default();
    let mut emu = Emulator::new();

    let mut wind = Window::new(100, 100, 800, 600, "trueLMAO");
    let mut but = Button::new(450, 350, 80, 40, "step1");
    let mut info = TextDisplay::new(0, 0, 600, 300, "asm");
    wind.end();
    wind.show();

    info.set_buffer(TextBuffer::default());
    let mut buffer = info.buffer().unwrap();

    let (s, r) = app::channel::<Update>();

    s.send(Update::Render);

    but.set_callback(move |_| {
        s.send(Update::Step);
        s.send(Update::Render);
    });

    let name = emu.core.mem.rom.domestic_name()
        .split_whitespace().collect::<Vec<&str>>().join(" ");

    wind.set_label(&format!("trueLMAO - {}", name));

    while app.wait() {
        while let Some(msg) = r.recv() {
            match msg {
                Update::Step => {
                    emu.step1();
                },
                Update::Render => {
                    let mut debug = String::new();
                    debug.push_str(&format!("PC: {:X}\n\n", emu.core.pc));
                    debug.push_str(&format!("D "));
                    for i in 0..=7 {
                        debug.push_str(&format!("{:X} ", emu.core.dar[i]));
                    }
                    debug.push_str(&format!("\n"));

                    debug.push_str(&format!("A "));
                    for i in 0..=7 {
                        debug.push_str(&format!("{:X} ", emu.core.dar[i + 7]));
                    }
                    debug.push_str(&format!("\n"));
                    debug.push_str(&format!("\n"));

                    for (pc, opcode) in emu.disasm() {
                        debug.push_str(&format!("0x{:X}\t{}\n", pc, opcode));
                    }
                    buffer.set_text(&debug);

                },
            }
        }
    }
}
