use r68k_emu::cpu::{STACK_POINTER_REG, ConfiguredCore};
use r68k_emu::interrupts::AutoInterruptController;
// use r68k_emu::ram::pagedmem::PagedMem;
// use r68k_tools::PC;
// use r68k_tools::memory::MemoryVec;
// use r68k_tools::disassembler::disassemble;

mod mem;
mod rom;

pub struct Emulator {
    core: ConfiguredCore<AutoInterruptController, mem::Mem>,
}

impl Emulator {
    // cpu
    // rom
    // ram
    // vdp
    pub fn new() -> Self {

        // orbtk/iced for proto ui
        // use a listing file

        let buf: Vec<u8> = include_bytes!("../../notes/res/s1.bin").to_vec();

        let mem = mem::Mem {
            rom: rom::Rom::from_vec(buf),
        };


        let int_ctrl = AutoInterruptController::new();
        let mut core = ConfiguredCore::new_with(0x206, int_ctrl, mem);

        core.pc = core.mem.rom.entry_point();
        core.dar[STACK_POINTER_REG] = core.mem.rom.stack_pointer();

        Emulator {
            core,
        }
    }

    pub fn step1(&mut self) {
        self.core.execute1();
    }

    pub fn disasm_stuff(&self) -> String {

        "test".to_string()
    }
}
