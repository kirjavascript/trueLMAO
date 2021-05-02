use fltk::{
    app,
    draw,
    button::Button,
    frame::Frame,
    input::IntInput,
    prelude::*,
    window::Window,
    text::{TextBuffer, TextDisplay},
};

use emu::Megadrive;

#[derive(Debug, Copy, Clone)]
pub enum Update {
    Render, Step, Frame
}

fn main() {
    let app = app::App::default();
    let buf: Vec<u8> = include_bytes!("./s1.bin").to_vec();

    let mut emu = Megadrive::new(buf);

    let mut wind = Window::new(100, 100, 800, 600, "trueLMAO");
    let mut but = Button::new(500, 350, 80, 40, "frame");
    let mut step = Button::new(500, 400, 80, 40, "step");
    let mut stepby = IntInput::new(500, 450, 80, 40, "step by");
    let mut info = TextDisplay::new(0, 0, 600, 300, "asm");
    stepby.set_value("1");

    let mut frame = Frame::new(0, 300, 16, 4, "");
    let mut framebuf: Vec<u8> = vec![0xFF; (16 * 4 * 4) as usize];

    let mut vram = Frame::new(600, 0, 80, 500, "");
    let mut vrambuf: Vec<u8> = vec![0xFF; (80 * 500 * 4) as usize];

    wind.end();
    wind.show();

    unsafe {
        draw::draw_rgba_nocopy(&mut frame, &framebuf);
        draw::draw_rgba_nocopy(&mut vram, &vrambuf);
    }

    frame.set_size(160,40);

    info.set_buffer(TextBuffer::default());
    let mut buffer = info.buffer().unwrap();

    let (s, r) = app::channel::<Update>();

    s.send(Update::Render);

    but.set_callback(move |_| {
        s.send(Update::Frame);
        s.send(Update::Render);
    });

    step.set_callback(move |_| {
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
                    emu.step_n(stepby.value().parse::<usize>().unwrap_or(1));
                },
                Update::Frame => {
                    emu.frame();
                },
                Update::Render => {
                    let mut debug = String::new();
                    debug.push_str(&format!("PC: {:X}\n\n", emu.core.pc));
                    // let v = emu.core.mem.vdp.VRAM.iter().map(|x|format!("{:X}", x)).collect::<Vec<String>>().join(" ");
                    // debug.push_str(&format!("VRAM: {}\n\n", v));
                    debug.push_str(&format!("D "));
                    for i in 0..=7 {
                        debug.push_str(&format!("{:X} ", emu.core.dar[i]));
                    }
                    debug.push_str(&format!("\n"));

                    debug.push_str(&format!("A "));
                    for i in 0..=7 {
                        debug.push_str(&format!("{:X} ", emu.core.dar[i + 8]));
                    }
                    debug.push_str(&format!("\n"));
                    debug.push_str(&format!("\n"));

                    for (pc, opcode) in emu.disasm() {
                        debug.push_str(&format!("0x{:X}\t{}\n", pc, opcode));
                    }
                    buffer.set_text(&debug);

                    // render CRAM

                    let cram_rgb = emu.core.mem.vdp.cram_rgb();

                    for (i, (red, green, blue)) in cram_rgb.iter().enumerate() {
                        let index = i * 4;
                        framebuf[index] = *red;
                        framebuf[index+1] = *green;
                        framebuf[index+2] = *blue;
                    }

                    // render VRAM

                    for (i, duxels) in emu.core.mem.vdp.VRAM.chunks(32).enumerate() {
                        let x_base = (i % 10) * 4 * 8;
                        let y_base = (i / 10) * 4 * 8 * 80;
                        let mut x = 0;
                        let mut y = 0;

                        let mut tile = vec![];
                        for duxel in duxels {
                            let first = (*duxel & 0xF0) >> 4;
                            let second = *duxel & 0xF;
                            tile.push(first);
                            tile.push(second);
                        }

                        for pixel in tile {
                            let (r, g, b) = cram_rgb[pixel as usize];
                            let base = x_base + y_base + x + y;

                            if base+2 > vrambuf.len() {
                                break;
                            }

                            vrambuf[base] = r;
                            vrambuf[base+1] = g;
                            vrambuf[base+2] = b;
                            x += 4;
                            if x >= (8 * 4) {
                                x = 0;
                                y += 80 * 4;
                            }
                        }
                    }


                    wind.redraw();
                },
            }
        }
    }
}
