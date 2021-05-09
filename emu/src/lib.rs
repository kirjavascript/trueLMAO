use r68k_emu::cpu::{STACK_POINTER_REG, ConfiguredCore};
use r68k_emu::interrupts::AutoInterruptController;

mod io;
mod mem;
mod rom;
mod vdp;
mod z80;

pub struct Megadrive {
    pub core: ConfiguredCore<AutoInterruptController, mem::Mem>,
    pub screen: [u8; 320 * 240 * 3],
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
            screen: [0; 320 * 240 * 3],
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

        self.clear_screen();

        self.core.mem.vdp.unset_status(vdp::VBLANK_MASK);
        self.core.mem.vdp.unset_status(vdp::VINT_MASK);

        let screen_height = self.core.mem.vdp.screen_height();
        let mut hint_counter = self.core.mem.vdp.hint_counter();
        for line in 0..screen_height {
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

            self.fire_beam(line);
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

    fn clear_screen(&mut self) {
        let bg_color = self.core.mem.vdp.bg_color();
        for pixel in self.screen.chunks_mut(3) {
            pixel[0] = bg_color.0;
            pixel[1] = bg_color.1;
            pixel[2] = bg_color.2;
        };
    }

    fn fire_beam(&mut self, line: usize) {
        let (cellw, cellh) = self.core.mem.vdp.scroll_size();
        let screen_width = self.core.mem.vdp.screen_width();

        // vscroll

        let columns = self.core.mem.vdp.registers[0xB] & 4 != 0;
        let offset = if columns {
            0 // screen_x * 2
        } else {
            0
        };

        let vscroll = &self.core.mem.vdp.VSRAM[offset..];

        let vscroll_a = vscroll[0] as usize;
        let vscroll_b = vscroll[1] as usize;


        println!("{} {:X} {:X}", columns, vscroll_a, vscroll_b);

        let tiles = |plane: &[u8]| {
            (0..cellw).map(|i| {
                let mut offset = i * 2;
                offset += (line / 8) * cellw as usize * 2;
                if offset as usize + 1 > plane.len() { panic!("offset > planelen") }

                let word = (plane[offset] as usize) << 8 | plane[offset + 1] as usize;
                let byte = word >> 8;
                let priority = byte >> 8;
                let tile = word & 0x7FF;
                let vflip = (byte & 0x10) != 0;
                let hflip = (byte & 0x8) != 0;
                let palette = (byte & 0x60) >> 5;

                (priority, palette, vflip, hflip, tile)
            }).collect::<Vec<_>>()
        };

        let (plane_a, plane_b) = self.core.mem.vdp.nametables();

        let tiles_a = tiles(&self.core.mem.vdp.VRAM[plane_a..]);
        let tiles_b = tiles(&self.core.mem.vdp.VRAM[plane_b..]);

        let (hscroll_a, hscroll_b) = self.core.mem.vdp.hscroll(line);

        let tile_y = line & 7;

        for screen_pixel in 0..screen_width {

            // TODO: switch to inner tile loop
            // TODO: draw_plane

            // TODO: correct overscan + vscroll

            let hoffset = (cellw * 8) - hscroll_b;
            let pixel = (screen_pixel + hoffset) % screen_width;
            let tile_index = ((screen_pixel + hoffset) / 8) % cellw;

            if let Some((_priority, palette, vflip, hflip, tile)) = tiles_b.get(tile_index) {
                let tile_pixel = if *hflip { pixel ^ 0xF } else { pixel };

                let x_offset = (tile_pixel & 6) >> 1;
                let y_offset = if *vflip { tile_y ^ 7 } else { tile_y } * 4;

                // tile is 32 bytes
                let px = self.core.mem.vdp.VRAM[(tile * 32) + x_offset + y_offset];

                let px = if tile_pixel & 1 == 0 {
                    px >> 4
                } else {
                    px & 0xF
                };

                if px != 0 {
                    let (r, g, b) = self.core.mem.vdp.color(*palette, px as _);

                    let screen_offset = (screen_pixel + (line * screen_width)) * 3;

                    self.screen[screen_offset] = r;
                    self.screen[screen_offset + 1] = g;
                    self.screen[screen_offset + 2] = b;
                }

            }

            let hoffset = (cellw * 8) - hscroll_a;
            let pixel = (screen_pixel + hoffset) % screen_width;
            let tile_index = ((screen_pixel + hoffset) / 8) % cellw;

            if let Some((_priority, palette, vflip, hflip, tile)) = tiles_a.get(tile_index) {
                let tile_pixel = if *hflip { pixel ^ 0xF } else { pixel };

                let x_offset = (tile_pixel & 6) >> 1;
                let y_offset = if *vflip { tile_y ^ 7 } else { tile_y } * 4;

                let px = self.core.mem.vdp.VRAM[(tile * 32) + x_offset + y_offset];

                let px = if tile_pixel & 1 == 0 {
                    px >> 4
                } else {
                    px & 0xF
                };

                if px != 0 {
                    let (r, g, b) = self.core.mem.vdp.color(*palette, px as _);

                    let screen_offset = (screen_pixel + (line * screen_width)) * 3;

                    self.screen[screen_offset] = r;
                    self.screen[screen_offset + 1] = g;
                    self.screen[screen_offset + 2] = b;
                }

            }
        }

    }
}
