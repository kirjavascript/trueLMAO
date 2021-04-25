use r68k_emu::cpu::{STACK_POINTER_REG, ConfiguredCore};
use r68k_emu::interrupts::AutoInterruptController;
// use r68k_emu::ram::pagedmem::PagedMem;
// use r68k_tools::PC;
// use r68k_tools::memory::MemoryVec;
// use r68k_tools::disassembler::disassemble;

mod mem;
mod rom;

pub struct Emulator {
    pub core: ConfiguredCore<AutoInterruptController, mem::Mem>,
}

impl Emulator {
    // cpu
    // rom
    // ram
    // vdp
    // z80
    pub fn new() -> Self {

        // IO trait for binding

        let buf: Vec<u8> = include_bytes!("../../notes/res/s1.bin").to_vec();

        let mem = mem::Mem::new(rom::Rom::from_vec(buf));

        let int_ctrl = AutoInterruptController::new();
        let mut core = ConfiguredCore::new_with(mem.rom.entry_point(), int_ctrl, mem);

        core.pc = core.mem.rom.entry_point();
        core.dar[STACK_POINTER_REG] = core.mem.rom.stack_pointer();

        Emulator {
            core,
        }
    }

    pub fn step1(&mut self) {
        self.core.execute1();
    }

    pub fn disasm(&self, pc: u32) -> (r68k_tools::PC, String) {
        use r68k_tools::PC;
        let m = r68k_tools::memory::MemoryVec::new8(PC(0), self.core.mem.rom.to_vec());
        let d = r68k_tools::disassembler::disassemble(PC(pc), &m);
        let (pc, opcode) = d.unwrap();

        (pc, opcode.to_string())
    }
}
