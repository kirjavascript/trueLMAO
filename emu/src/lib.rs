use r68k_emu::cpu::ConfiguredCore;
use r68k_emu::cpu::Core;
use r68k_emu::interrupts::AutoInterruptController;
use r68k_emu::ram::pagedmem::PagedMem;
use r68k_tools::PC;
use r68k_tools::memory::MemoryVec;
use r68k_tools::disassembler::disassemble;

mod rom;

pub struct Emulator {
    cpu: ConfiguredCore<AutoInterruptController, PagedMem>,
}

impl Emulator {
    // cpu
    // rom
    // ram
    // vdp
    pub fn new() -> Self {

        // orbtk for proto ui
        // use a listing file

    // let mut buf: Vec<u8> = Vec::new();
    // File::open("./res/s1.bin").unwrap().read_to_end(&mut buf);

        let buf: Vec<u8> = include_bytes!("../../res/s1.bin").to_vec();

        let int_ctrl = AutoInterruptController::new();
        let mut mem = PagedMem::new(0);
        for (i, data) in buf.iter().enumerate() {
            mem.write_u8(i as u32, *data as u32);
        }
        let mut r68k = ConfiguredCore::new_with(0x206, int_ctrl, mem);


    //// r68k.pc = 0x206;
    //// r68k.resume_normal_processing();
    //println!("PC is 0x{:06x}", r68k.pc);
    //let cycle = r68k.execute1();
    //println!("PC is 0x{:06x}", r68k.pc);


    ////
    //println!("{:?}", cycle);
    //let mem = MemoryVec::new8(PC(0), buf.clone());
    //let res = disassemble(PC(0x206), &mem);
    //println!("{}", res.unwrap().1);

        Emulator {
            cpu: r68k,
        }
    }

    pub fn step1(&mut self) {
        self.cpu.pc = 0x206;
        self.cpu.execute1();
    }

    pub fn disasm_stuff(&self) -> String {

        "test".to_string()
    }
}
