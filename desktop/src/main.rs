use fltk::{
    app,
    draw,
    button::Button,
    frame::Frame,
    prelude::*,
    window::Window,
    text::{TextBuffer, TextDisplay},
    image,
    enums,
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
    let mut but = Button::new(450, 350, 80, 40, "step1001");
    let mut info = TextDisplay::new(0, 0, 600, 300, "asm");

    let mut frame = Frame::new(0, 300, 16, 4, "");
    let mut framebuf: Vec<u8> = vec![0xFF; (16 * 4 * 4) as usize];

    // for (i, pixel) in framebuf.chunks_exact_mut(4).enumerate() {
    //     pixel.copy_from_slice(& [0x26, 0x00, 0x33, 0xff]);
    // }
    //


    framebuf[0] = 0;
    framebuf[1] = 0;
    framebuf[2] = 0;

    wind.end();
    wind.show();

    unsafe {
        draw::draw_rgba_nocopy(&mut frame, &framebuf);
    }

    frame.set_size(160,40);

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

                    let c = |x| (x << 4) | x;

                    for (i, color) in emu.core.mem.vdp.CRAM.iter().enumerate() {
                        let index = i * 4;
                        let red = color & 0xF;
                        let green = (color & 0xF0) >> 4;
                        let blue = (color & 0xF00) >> 8;
                        framebuf[index] = c(red as u8);
                        framebuf[index+1] = c(green as u8);
                        framebuf[index+2] = c(blue as u8);
                    }
                },
                Update::Render => {
                    let mut debug = String::new();
                    debug.push_str(&format!("PC: {:X}\n\n", emu.core.pc));
                    let v = emu.core.mem.vdp.CRAM[0..64].iter().map(|x|format!("{:X}", x)).collect::<Vec<String>>().join(" ");
                    debug.push_str(&format!("CRAM: {}\n\n", v));
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




                    wind.redraw();
                },
            }
        }
    }
}
