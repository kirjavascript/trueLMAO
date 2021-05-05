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

        let hscroll_addr = self.core.mem.vdp.hscroll_addr();



        let vscroll_mode = self.core.mem.vdp.vscroll_mode();
        let hscroll_mode = self.core.mem.vdp.registers[0xB] & 3;
        let planea_nametable =
            ((self.core.mem.vdp.registers[4] >> 3) as u32 & 3) << 10;


        let index = match hscroll_mode {
            0 => 0,
            1 => line & 7,
            2 => line & 0xFFF8,
            3 => line,
            _ => unreachable!(),
        };

        let hscroll = hscroll_addr + (index * 4); // 3 ?

        let planeb_nametable =
            (self.core.mem.vdp.registers[4] as u32 & 3) << 10;

        let planeb = &self.core.mem.vdp.VRAM[planea_nametable as usize..];


        let tiles = (0..cellw).map(|i| {
            let mut offset = i as usize * 2;
            offset += (line / 8) * cellw as usize * 2;
            if i as usize + 1 > planeb.len() { return 0 }
            (planeb[offset] as usize & 7) << 8 | planeb[offset + 1] as usize
        }).collect::<Vec<_>>();

        // println!("{:?} {:?}", tiles, tiles.len());

        let tile_y = line & 7;
        let y_offset = tile_y * 4;

        // let y_offset = 0;


        // for i in 0..0xFF {
        //     print!("{:b} ", planeb[i]);
        // }

        // println!("");

        // tile is 32 bytes

        for pixel in 0..screen_width {
            if let Some(tile) = tiles.get(pixel / 8) {
                let x_offset = (pixel & 6) >> 1;
                let px = self.core.mem.vdp.VRAM[(tile * 32) + x_offset + y_offset];

                let px = if pixel & 1 == 0 {
                    px >> 4
                } else {
                    px & 0xF
                };

                let (r, g, b) = self.core.mem.vdp.color(0, px as _);

                let screen_offset = (pixel + (line * screen_width)) * 3;

                self.screen[screen_offset] = r;
                self.screen[screen_offset + 1] = g;
                self.screen[screen_offset + 2] = b;
            }
        }





    }
}
