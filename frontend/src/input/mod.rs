pub fn dummy_input(ctx: &egui::Context, emu: &mut emu::Megadrive) {
    let mut value = 0;
    if ctx.input().key_down(egui::Key::Enter) {
        value += emu::io::Gamepad::START;
    }
    if ctx.input().key_down(egui::Key::W) {
        value += emu::io::Gamepad::U;
    }
    if ctx.input().key_down(egui::Key::S) {
        value += emu::io::Gamepad::D;
    }
    if ctx.input().key_down(egui::Key::A) {
        value += emu::io::Gamepad::L;
    }
    if ctx.input().key_down(egui::Key::D) {
        value += emu::io::Gamepad::R;
    }
    if ctx.input().key_down(egui::Key::J) {
        value += emu::io::Gamepad::A;
    }
    if ctx.input().key_down(egui::Key::K) {
        value += emu::io::Gamepad::B;
    }
    if ctx.input().key_down(egui::Key::L) {
        value += emu::io::Gamepad::C;
    }

    emu.core.mem.io.gamepad[0].set(value);
}
