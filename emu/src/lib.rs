use r68k_emu::cpu::{STACK_POINTER_REG, ConfiguredCore};
use r68k_emu::interrupts::AutoInterruptController;

mod io;
mod mem;
mod rom;
mod vdp;
mod z80;

pub struct Megadrive {
    pub core: ConfiguredCore<AutoInterruptController, mem::Mem>,
}

impl Megadrive {
    pub fn new(buf: Vec<u8>) -> Self {
        let mem = mem::Mem::new(buf.into());

        let int_ctrl = AutoInterruptController::new();
        let mut core = ConfiguredCore::new_with(mem.rom.entry_point(), int_ctrl, mem);

        core.dar[STACK_POINTER_REG] = core.mem.rom.stack_pointer();

        Megadrive {
            core,
        }
    }

    pub fn step_n(&mut self, amount: usize) {
        for _ in 0..amount {
            self.core.execute1();
        }
    }

    pub fn frame(&mut self) {
        // https://segaretro.org/Sega_Mega_Drive/Technical_specifications#Graphics

        let screen_width = if self.core.mem.vdp.registers[12] & 0x01 > 0 {
            320
        } else {
            256
        };
        let screen_height = if self.core.mem.vdp.registers[1] & 0x08 > 0 {
            240
        } else {
            224
        };

        self.core.mem.vdp.status &= !8; // clear vblank

        let mut hint_counter = self.core.mem.vdp.registers[10] as isize;
        for line in 0..screen_height {
            self.core.execute(2680);

            hint_counter -= 1;
            if hint_counter < 0 {
                hint_counter = self.core.mem.vdp.registers[10] as isize;

                if self.core.mem.vdp.registers[0] & 0x10 > 0 {
                    self.core.int_ctrl.request_interrupt(4);
                }

            }

            self.core.mem.vdp.status |= 4;
            self.core.execute(636);
            self.core.mem.vdp.status &= !4;

            self.core.execute(104);

            // render
        }

        self.core.mem.vdp.status |= 4;

        self.core.execute(588);

        self.core.mem.vdp.status |= 0x80;

        self.core.execute(200);

        if self.core.mem.vdp.registers[1] & 0x20 > 0 {
            self.core.int_ctrl.request_interrupt(6);
        }

        self.core.execute(3420-788);

        for _ in screen_height..262 {
            self.core.execute(3420);
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
}
