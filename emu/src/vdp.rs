use crate::mem::Mem;

#[allow(non_snake_case)]
pub struct VDP {
    pub VRAM: [u8; 0x10000],
    pub CRAM: [u16; 0x40],
    pub VSRAM: [u16; 0x40],
    pub registers: [u8; 0x20],
    pub status: u32,
    pub control_code: u32,
    pub control_address: u32,
    pub control_pending: bool,
    pub dma_pending: bool,
}

pub const VBLANK_MASK: u32 = 8;
pub const HBLANK_MASK: u32 = 4;
pub const VINT_MASK: u32 = 0x80;

pub enum VDPType {
    VRAM, CRAM, VSRAM,
}

impl From<u32> for VDPType {
    fn from(value: u32) -> Self {
        match value {
            0 | 1 => VDPType::VRAM,
            2 | 3 => VDPType::CRAM,
            4 | 5 => VDPType::VSRAM,
            _ => unreachable!("VDPType {:X}", value),
        }
    }
}

pub struct Sprite {
    pub tile: usize,
    pub x_pos: usize,
    pub y_pos: usize,
    pub width: usize,
    pub height: usize,
    pub palette: usize,
    pub priority: usize,
    pub v_flip: bool,
    pub h_flip: bool,
}

impl Sprite {
    pub fn x_coord(&self) -> isize { (self.x_pos as isize)-128 }
    pub fn y_coord(&self) -> isize { (self.y_pos as isize)-128 }
}

fn cram_to_rgb(color: u16) -> (u8, u8, u8) {
    let red = color & 0xf;
    let green = (color & 0xf0) >> 4;
    let blue = (color & 0xf00) >> 8;
    let dupe = |x| (x << 4) | x;
    (dupe(red as u8), dupe(green as u8), dupe(blue as u8))
}

impl VDP {
    pub fn new() -> Self {
        Self {
            VRAM: [0; 0x10000],
            CRAM: [0; 0x40],
            VSRAM: [0; 0x40],
            registers: [0; 0x20],
            status: 0x3400,
            control_code: 0,
            control_address: 0,
            control_pending: false,
            dma_pending: false,
        }
    }

    pub fn color(&self, line: usize, index: usize) -> (u8, u8, u8) {
        cram_to_rgb(self.CRAM[index + (line * 0x10)])
    }

    pub fn bg_color(&self) -> (u8, u8, u8) {
        let vdp_bg = self.registers[7];
        let index = vdp_bg & 0xF;
        let line = (vdp_bg >> 4) & 3;
        self.color(line as _, index as _)
    }

    pub fn cram_rgb(&self) -> [(u8, u8, u8); 64] {
        let mut rgb = [(0, 0, 0); 64];
        for (i, color) in self.CRAM.iter().enumerate() {
            rgb[i] = cram_to_rgb(*color);
        }
        rgb
    }

    pub fn screen_width(&self) -> usize {
        if self.registers[12] & 1 > 0 { 320 } else { 256 }
    }

    pub fn screen_height(&self) -> usize {
        if self.registers[1] & 0x08 > 0 { 240 } else { 224 }
    }

    pub fn hint_counter(&self) -> isize {
        self.registers[0xA] as isize
    }

    pub fn hint_enabled(&self) -> bool {
        self.registers[0] & 0x10 != 0
    }

    pub fn vint_enabled(&self) -> bool {
        self.registers[1] & 0x20 != 0
    }

    pub fn dma_length(&self) -> u32 {
        self.registers[0x13] as u32 | ((self.registers[0x14] as u32) << 8)
    }

    pub fn set_status(&mut self, mask: u32) {
        self.status |= mask;
    }

    pub fn unset_status(&mut self, mask: u32) {
        self.status &= !mask;
    }

    pub fn scroll_size(&self) -> (usize, usize) {
        let to_cells = |size| (size as usize + 1) * 32;
        // TODO: 96 is invalid
        (
            to_cells(self.registers[0x10] & 3),
            to_cells((self.registers[0x10] >> 4) & 3),
        )
    }

    pub fn nametables(&self) -> (usize, usize) {
        let plane_a = ((self.registers[2] >> 3) as usize) * 0x2000;
        let plane_b = (self.registers[4] as usize) * 0x2000;
        (plane_a, plane_b)
    }

    pub fn cell40(&self) -> bool {
        (self.registers[0xC] as usize) >> 7 == 1
    }

    pub fn sprites(&self, screen_y: usize) -> Vec<Sprite> {
        let mask = if self.cell40() { 0x7F } else { 0x7E };
        let addr = ((self.registers[5] as usize) & mask) << 9;

        let mut index = 0usize;
        let mut sprites = vec![];
        loop {
            let sprite_screen_y = screen_y + 128;
            let offset = addr + (index * 8);
            let sprite = &self.VRAM[offset..];
            let next = sprite[3].into();
            let y_pos = ((sprite[0] as usize) << 8) | sprite[1] as usize;
            let height = (sprite[2] as usize & 3) + 1;
            if sprite_screen_y >= y_pos && sprite_screen_y < y_pos + (height * 8) {
                let width = (sprite[2] as usize >> 2) + 1;
                let priority = sprite[4] as usize >> 7;
                let palette  = sprite[4] as usize >> 5 & 3;
                let v_flip = sprite[4] as usize >> 4 & 1 == 1;
                let h_flip = sprite[4] as usize >> 3 & 1 == 1;
                let tile = (((sprite[4] as usize & 7) << 8) | sprite[5] as usize) * 0x20;
                let x_pos = ((sprite[6] as usize) << 8) | sprite[7] as usize;
                sprites.push(Sprite {
                    y_pos,
                    width,
                    height,
                    priority,
                    palette,
                    v_flip,
                    h_flip,
                    tile,
                    x_pos,
                });
            }


            index = next;

            if index == 0 || sprites.len() == if cell40 { 80 } else { 64 } {
                break;
            }
        }

        sprites
    }

    pub fn hscroll(&self, screen_y: usize) -> (usize, usize) {
        let addr = (self.registers[0xD] as usize & 0x3F) << 10;
        let mode = self.registers[0xB] & 3;

        let index = match mode {
            0 => 0,
            2 => screen_y & 0xFFF8,
            3 => screen_y,
            _ => unreachable!("invalid hscroll"),
        };

        let hscroll = &self.VRAM[addr + (index * 4)..];

        let hscroll_a = ((hscroll[0] as usize) << 8) + hscroll[1] as usize;
        let hscroll_b = ((hscroll[2] as usize) << 8) + hscroll[3] as usize;
        (hscroll_a, hscroll_b)
    }

    pub fn vscroll(&self, screen_x: usize) -> &[u16] {
        let columns = self.registers[0xB] & 4 != 0;
        let offset = if columns {
            screen_x * 2
        } else {
            0
        };

        // 0 is A, 1 is B

        &self.VSRAM[offset..]
    }

    pub fn autoinc(&self) -> u32 {
        self.registers[15] as _
    }

    pub fn read(&self, mut address: u32) -> u32 {
        address &= 0x1F;

        if (0x4..=0x7).contains(&address) {
            return self.status;
        }

        println!("TODO: vdp read {:X}", address);
        0
    }

    pub fn write(mem: &mut Mem, mut address: u32, value: u32) {
        address &= 0x1F;
        if address < 0x4 {
            mem.vdp.write_data_port(value);
        } else if address < 0x8 {
            VDP::write_control_port(mem, value);
        } else {
            todo!("vdp write {:X} {:X}", address, value);
        }
    }

    fn write_data(&mut self, target: VDPType, value: u32) {
        match target {
            VDPType::VRAM => {
                self.VRAM[self.control_address as usize] = ((value >> 8) & 0xff) as _;
                self.VRAM[self.control_address as usize + 1] = (value & 0xff) as _;
            },
            VDPType::CRAM => {
                self.CRAM[((self.control_address & 0x7f) >> 1) as usize] = value as _;
            },
            VDPType::VSRAM => {
                self.VSRAM[((self.control_address & 0x7f) >> 1) as usize] = value as _;
            },
        }
    }

    fn write_data_port(&mut self, value: u32) {
        if self.control_code & 1 == 1 {
            self.write_data(VDPType::from(self.control_code & 0xE), value);
        }
        self.control_address = (self.control_address + self.autoinc()) & 0xFFFF;
        self.control_pending = false;

        if self.dma_pending {
            self.dma_pending = false;
            for _ in 0..self.dma_length() {
                self.VRAM[self.control_address as usize] = (value >> 8) as _;
                self.control_address += self.autoinc();
                self.control_address &= 0xFFFF;
            }
        }
    }

    fn write_control_port(mem: &mut Mem, value: u32) {
        if mem.vdp.control_pending {
            mem.vdp.control_code = (mem.vdp.control_code & 3) | ((value >> 2) & 0x3c);
            mem.vdp.control_address = (mem.vdp.control_address & 0x3fff) | ((value & 3) << 14);
            mem.vdp.control_pending = false;

            if mem.vdp.control_code & 0x20 > 0 && mem.vdp.registers[1] & 0x10 > 0 {
                if (mem.vdp.registers[23] >> 6) == 2 && (mem.vdp.control_code & 7) == 1 {
                    mem.vdp.dma_pending = true;
                } else if (mem.vdp.registers[23] as u32 >> 6) == 3 {
                   todo!("DMA copy");
                } else {
                    let mut source =
                        ((mem.vdp.registers[21] as u32) << 1)
                        | ((mem.vdp.registers[22] as u32) << 9)
                        | ((mem.vdp.registers[23] as u32) << 17);

                    for _ in 0..mem.vdp.dma_length() {
                        let word = mem.read_u16(source);
                        source += 2;
                        mem.vdp.write_data(VDPType::from(mem.vdp.control_code & 0x7), word);
                        mem.vdp.control_address += mem.vdp.autoinc();
                        mem.vdp.control_address &= 0xFFFF;
                    }

                }
            }
        } else {
            if value & 0xc000 == 0x8000 {
                let (register, value) = ((value >> 8) & 0x1F, value & 0xFF);
                if mem.vdp.registers[1] & 4 > 0 || register <= 10 {
                    mem.vdp.registers[register as usize] = value as u8;
                }
                mem.vdp.control_code = 0;
            } else {
                mem.vdp.control_code = (mem.vdp.control_code & 0x3c) | ((value >> 14) & 3);
                mem.vdp.control_address = (mem.vdp.control_address & 0xc000) | (value & 0x3fff);
                mem.vdp.control_pending = true;
            }
        }
    }

}
