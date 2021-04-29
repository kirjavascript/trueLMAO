use r68k_emu::cpu::{STACK_POINTER_REG, ConfiguredCore};
use r68k_emu::interrupts::AutoInterruptController;

mod io;
mod mem;
mod rom;
mod vdp;
mod z80;

pub struct Emulator {
    pub core: ConfiguredCore<AutoInterruptController, mem::Mem>,
}

impl Emulator {
    pub fn new() -> Self {
        let buf: Vec<u8> = include_bytes!("./test.bin").to_vec();

        let mem = mem::Mem::new(buf.into());

        let int_ctrl = AutoInterruptController::new();
        let mut core = ConfiguredCore::new_with(mem.rom.entry_point(), int_ctrl, mem);

        core.dar[STACK_POINTER_REG] = core.mem.rom.stack_pointer();

        Emulator {
            core,
        }
    }

    pub fn step1(&mut self) {
        for _ in 0..10001 {

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
}
