use fltk::{
    app,
    button::Button,
    frame::Frame,
    prelude::*,
    window::Window,
    text::{TextBuffer, TextDisplay},
};

use emu::Emulator;

fn main() {
    let app = app::App::default();
    let mut wind = Window::new(100, 100, 400, 300, "trueLMAO");
    let mut but = Button::new(230, 210, 80, 40, "cool");
    let mut disasm = TextDisplay::new(0, 0, 400, 300, "asm");
    disasm.set_buffer(TextBuffer::default());
    let emu = Emulator::new();
    let mut out = String::new();
    let (pc, opcode) = emu.disasm(emu.core.pc);
    out.push_str(&opcode); out.push_str("\n");
    let (pc, opcode) = emu.disasm(pc.0);
    out.push_str(&opcode); out.push_str("\n");
    let (pc, opcode) = emu.disasm(pc.0);
    out.push_str(&opcode); out.push_str("\n");
    let (pc, opcode) = emu.disasm(pc.0);
    out.push_str(&opcode); out.push_str("\n");
    let (pc, opcode) = emu.disasm(pc.0);
    out.push_str(&opcode); out.push_str("\n");
    let (pc, opcode) = emu.disasm(pc.0);
    out.push_str(&opcode); out.push_str("\n");
    let (pc, opcode) = emu.disasm(pc.0);
    out.push_str(&opcode); out.push_str("\n");
    let (pc, opcode) = emu.disasm(pc.0);
    out.push_str(&opcode); out.push_str("\n");
    let (pc, opcode) = emu.disasm(pc.0);
    out.push_str(&opcode); out.push_str("\n");
    let (pc, opcode) = emu.disasm(pc.0);
    out.push_str(&opcode); out.push_str("\n");
    let (pc, opcode) = emu.disasm(pc.0);
    out.push_str(&opcode); out.push_str("\n");
    let (pc, opcode) = emu.disasm(pc.0);
    out.push_str(&opcode); out.push_str("\n");
    let (pc, opcode) = emu.disasm(pc.0);
    out.push_str(&opcode); out.push_str("\n");

    disasm.buffer().unwrap().set_text(&out);
    wind.end();
    wind.show();
    // but.set_callback(move |_| frame.set_label("Hello World!"));
    app.run().unwrap();
}
