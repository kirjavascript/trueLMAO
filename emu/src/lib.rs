use r68k_emu::cpu::{STACK_POINTER_REG, ConfiguredCore};
use r68k_emu::interrupts::AutoInterruptController;

pub mod gfx;
pub mod io;
pub mod mem;
pub mod rom;
pub mod vdp;
pub mod z80;

use gfx::Gfx;

pub struct Megadrive {
    pub core: ConfiguredCore<AutoInterruptController, mem::Mem>,
    pub gfx: Gfx,
    // version: NTSC/PAL
}

impl Megadrive {
    pub fn new(buf: Vec<u8>) -> Self {
        let mem = mem::Mem::new(buf.into());

        let int_ctrl = AutoInterruptController::new();
        let mut core = ConfiguredCore::new_with(mem.rom.entry_point(), int_ctrl, mem);

        core.dar[STACK_POINTER_REG] = core.mem.rom.stack_pointer();

        Megadrive {
            core,
            gfx: Gfx::new(),
        }

    }

    pub fn step_n(&mut self, amount: usize) {
        for _ in 0..amount {
            self.core.execute1();
        }
    }

    pub fn disasm(&self) -> Vec<(u32, String)> {
        use r68k_tools::PC;
        let mut buffer = Vec::new();
        let mut opcodes = Vec::new();
        // longest opcode is 16 bytes
        for i in 0..(16 * 10) {
            buffer.push(self.core.mem.rom.read_byte(self.core.pc + i));
        }
        let mvec = r68k_tools::memory::MemoryVec::new8(PC(0), buffer);
        let mut cursor = PC(0);
        for _ in 0..10 {
            let disasm = r68k_tools::disassembler::disassemble(cursor, &mvec);
            if let Ok((pc, opcode)) = disasm {
                opcodes.push((cursor.0 + self.core.pc, opcode.to_string().to_lowercase()));
                cursor = pc;
            }
        }

        opcodes
    }

    pub fn frame(&mut self) {
        /* cycle counts initially taken from drx/kiwi */
        // TODO: use a counter instead
        // TODO: patch gfx.screen_width here for gfx.draw()

        Gfx::clear_screen(self);

        self.core.mem.vdp.unset_status(vdp::VBLANK_MASK);
        self.core.mem.vdp.unset_status(vdp::VINT_MASK);

        let screen_height = self.core.mem.vdp.screen_height();
        let mut hint_counter = self.core.mem.vdp.hint_counter();
        for screen_y in 0..screen_height {
            self.core.execute(2680);

            hint_counter -= 1;
            if hint_counter < 0 {
                hint_counter = self.core.mem.vdp.hint_counter();

                if self.core.mem.vdp.hint_enabled() {
                    self.core.int_ctrl.request_interrupt(4);
                }

            }

            self.core.mem.vdp.set_status(vdp::HBLANK_MASK);
            self.core.execute(636);
            self.core.mem.vdp.unset_status(vdp::HBLANK_MASK);

            self.core.execute(104);

            self.fire_beam(screen_y);
        }

        self.core.mem.vdp.set_status(vdp::VBLANK_MASK);

        self.core.execute(588);

        if self.core.mem.vdp.vint_enabled() {
            self.core.int_ctrl.request_interrupt(6);
            self.core.mem.vdp.set_status(vdp::VINT_MASK);
        }

        self.core.execute(3420-588);

        for _ in screen_height..262 {
            self.core.execute(3420);
        }
    }

    fn fire_beam(&mut self, screen_y: usize) {
        let (cell_w, cell_h) = self.core.mem.vdp.scroll_size();
        let (plane_a, plane_b) = self.core.mem.vdp.nametables();
        let (hscroll_a, hscroll_b) = self.core.mem.vdp.hscroll(screen_y);
        let screen_width = self.core.mem.vdp.screen_width();
        let sprites = self.core.mem.vdp.sprites(screen_y);

        // (unused in the plane drawing
        // TODO: improve perf by having two buffers to render to and combine them, doing both
        // priorities at once. OR have a write queue

        // plane B, low priority
        Gfx::draw_plane_line(
            self,
            cell_w,
            cell_h,
            screen_y,
            screen_width,
            plane_b,
            hscroll_b,
            1, // vscroll_offset
            0, // priority
        );

        // plane A, low priority
        Gfx::draw_plane_line(
            self,
            cell_w,
            cell_h,
            screen_y,
            screen_width,
            plane_a,
            hscroll_a,
            0,
            0,
        );

        // sprites, low priority
        Gfx::draw_sprite_line(
            self,
            &sprites,
            screen_y,
            screen_width,
            0, // priority
        );

        // window, low priority
        Gfx::draw_window_line(
            self,
            screen_y,
            screen_width,
            0, // priority
        );

        // plane B, high priority
        Gfx::draw_plane_line(
            self,
            cell_w,
            cell_h,
            screen_y,
            screen_width,
            plane_b,
            hscroll_b,
            1,
            1,
        );

        // plane A, high priority
        Gfx::draw_plane_line(
            self,
            cell_w,
            cell_h,
            screen_y,
            screen_width,
            plane_a,
            hscroll_a,
            0,
            1,
        );

        // sprites, high priority
        Gfx::draw_sprite_line(
            self,
            &sprites,
            screen_y,
            screen_width,
            1,
        );

        // window, high priority
        Gfx::draw_window_line(
            self,
            screen_y,
            screen_width,
            1, // priority
        );

    }

}
