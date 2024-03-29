use crate::Megadrive;

pub fn disasm_demo(emu: &Megadrive) -> Vec<(u32, String)> {
    use r68k_tools::PC;
    let mut buffer = Vec::new();
    let mut opcodes = Vec::new();
    // longest opcode is 16 bytes
    for i in 0..(16 * 10) {
        buffer.push(emu.core.mem.rom.read_byte(emu.core.pc + i));
    }
    let mvec = r68k_tools::memory::MemoryVec::new8(PC(0), buffer);
    let mut cursor = PC(0);
    for _ in 0..10 {
        let disasm = r68k_tools::disassembler::disassemble(cursor, &mvec);
        if let Ok((pc, opcode)) = disasm {
            opcodes.push((cursor.0 + emu.core.pc, opcode.to_string().to_lowercase()));
            cursor = pc;
        }
    }

    opcodes

}
