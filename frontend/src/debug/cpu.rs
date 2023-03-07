pub fn cpu_window(ctx: &egui::Context, emu: &emu::Megadrive) {
    egui::Window::new("cpu")
        .min_width(800.)
        .show(ctx, |ui| {
            let mut debug = String::new();
            debug.push_str(&format!("PC: {:X}\n\n", emu.core.pc));


            // let v = emu.core.mem.vdp.VSRAM.iter().map(|x|format!("{:X}", x)).collect::<Vec<String>>().join(" ");
            // debug.push_str(&format!("VSRAM: {}\n\n", v));

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

            for (pc, opcode) in emu::debug::disasm_demo(&emu) {
                debug.push_str(&format!("0x{:X}\t{}\n", pc, opcode));
            }
            ui.label(&debug);
        });
}
