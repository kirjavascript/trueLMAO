use super::Megadrive;

pub type Screen = [u8; 320 * 240 * 3];

pub struct Gfx {
    pub screen: Screen,
}

impl Gfx {
    pub fn new() -> Self {
        Gfx {
            screen: [0; 320 * 240 * 3],
        }
    }

    pub fn clear_screen(emu: &mut Megadrive) {
        let bg_color = emu.core.mem.vdp.bg_color();
        for pixel in emu.gfx.screen.chunks_mut(3) {
            pixel[0] = bg_color.0;
            pixel[1] = bg_color.1;
            pixel[2] = bg_color.2;
        };
    }

    pub fn line_slice(&mut self, screen_y: usize) -> &mut [u8] {
        // TODO: screen_width
        let offset = screen_y * 320 * 3;
        &mut self.screen[offset..offset + (320 * 3)]
    }

    pub fn draw_plane_line<'a>(
        emu: &'a mut Megadrive,
        mut line_high: &'a mut [u8],
        cram_rgb: &[(u8, u8, u8); 64],
        cell_w: usize,
        cell_h: usize,
        screen_y: usize,
        screen_width: usize,
        nametable: usize,
        hscroll: usize,
        vscroll_offset: usize, // 0 is A, 1 is B
    ) {
        let mut line_low = emu.gfx.line_slice(screen_y);

        let columns_mode = emu.core.mem.vdp.vcolumns_mode();
        let vscroll_base = emu.core.mem.vdp.VSRAM[vscroll_offset] as usize;

        let plane_width = cell_w * 8;
        let plane_height = cell_h * 8;

        let mut screen_x = 0;
        let mut target = 0;
        let inc = if columns_mode { 16 } else { screen_width };

        while target < screen_width {
            target += inc;

            let vscroll = if columns_mode {
                let column_offset = (screen_x / 16) * 2;
                emu.core.mem.vdp.VSRAM[vscroll_offset + column_offset] as usize
            } else {
                vscroll_base
            };

            let mut first_item = true;

            while screen_x < target {
                let mut start = 0;
                let end = 8.min(target - screen_x);
                let mut width = end;

                if first_item {
                    let hoff = hscroll % 8;
                    if hoff > 0 {
                        start = 8 - hoff;
                        width = hoff;
                    }
                    first_item = false;
                }

                let start = start;
                let width = width;

                let hscroll_rem = hscroll % plane_width;
                let x_offset = (screen_x + plane_width - hscroll_rem) % plane_width;
                let y_offset = (screen_y + vscroll) % plane_height;

                let tile_index = ((x_offset / 8) + (y_offset / 8 * cell_w)) * 2;

                let byte = emu.core.mem.vdp.VRAM[nametable + tile_index] as usize;
                let next = emu.core.mem.vdp.VRAM[nametable + tile_index + 1] as usize;
                let word = byte << 8 | next;

                let tile = word & 0x7FF;
                let vflip = (byte & 0x10) != 0;
                let hflip = (byte & 0x8) != 0;
                let palette = (byte & 0x60) >> 5;
                let priority = (byte >> 7) & 1;

                let y = y_offset & 7;
                let y = if vflip { y ^ 7 } else { y };
                let index = (tile * 32) + (y * 4);

                let mut pixels = [0; 8];
                let mut pos = 0;
                for duxel in &emu.core.mem.vdp.VRAM[index..index+4] {
                    let abs_pos = if hflip { 7 - pos } else { pos };
                    pixels[abs_pos] = duxel >> 4;
                    let abs_pos = if hflip { 7 - (pos + 1) } else { pos + 1 };
                    pixels[abs_pos] = duxel & 0xF;
                    pos += 2;
                }

                let target = if priority == 0 { &mut line_low } else { &mut line_high };

                for (x, px) in (&pixels[start..end]).iter().enumerate() {
                    if *px != 0 {
                        let (r, g, b) = cram_rgb[*px as usize + (palette * 0x10)];
                        let offset = (screen_x + x) * 3;
                        (*target)[offset] = r;
                        (*target)[offset + 1] = g;
                        (*target)[offset + 2] = b;
                    }
                }

                screen_x += width;
            };
        }
    }

    //

    pub fn draw_sprite_line<'a>(
        emu: &'a mut Megadrive,
        mut line_high: &'a mut [u8],
        cram_rgb: &[(u8, u8, u8); 64],
        screen_y: usize,
        screen_width: usize,
    ) {
        let mut line_low = emu.gfx.line_slice(screen_y);

        let sprites = emu.core.mem.vdp.sprites(screen_y);

        for sprite in sprites.iter().rev() {
            let target = if sprite.priority == 0 { &mut line_low } else { &mut line_high };
            let sprite_y = screen_y as isize - sprite.y_coord();
            let tiles = &emu.core.mem.vdp.VRAM[sprite.tile..];
            for sprite_x in 0..sprite.width * 8 {
                let x_offset = sprite.x_coord() + sprite_x as isize;

                if x_offset >= 0 && x_offset < screen_width as isize {

                    let sprite_base_x = if sprite.h_flip { sprite_x ^ 7 } else { sprite_x };
                    let x_base = (sprite_base_x & 6) >> 1;
                    let y_base = sprite_y & 7;
                    let y_base = if sprite.v_flip { y_base ^ 7 } else { y_base } * 4;

                    let tile_offset = (x_base as usize) + y_base as usize;

                    let x_tile = sprite_x as usize / 8;
                    let x_tile = if sprite.h_flip { sprite.width - x_tile - 1} else { x_tile };
                    let y_tile = sprite_y as usize / 8;
                    let y_tile = if sprite.v_flip { sprite.height - y_tile - 1} else { y_tile };
                    let extra = (y_tile * 32) + (x_tile * 32 * sprite.height);

                    let px = tiles[tile_offset + extra];
                    let px = if sprite_base_x & 1 == 0 { px >> 4 } else { px & 0xF };

                    if px != 0 {
                        let (r, g, b) = cram_rgb[px as usize + (sprite.palette * 0x10)];
                        let offset = x_offset as usize * 3;

                        if offset + 2 <= target.len() {
                            (*target)[offset] = r;
                            (*target)[offset + 1] = g;
                            (*target)[offset + 2] = b;
                        }
                    }
                }
            }
        }
    }


    pub fn draw_window_line<'a>(
        emu: &'a mut Megadrive,
        mut line_high: &'a mut [u8],
        cram_rgb: &[(u8, u8, u8); 64],
        screen_y: usize,
    ) {
        let mut line_low = emu.gfx.line_slice(screen_y);
        // TODO: support non-320 size nametable
        // TODO: plane A / window exclusivity (perf)
        let nametable = (emu.core.mem.vdp.registers[3] as usize >> 1) * 0x800;
        let window_x = emu.core.mem.vdp.registers[0x11];
        let window_y = emu.core.mem.vdp.registers[0x12];
        let window_left = window_x >> 7 == 0;
        let window_top =  window_y >> 7 == 0;
        let window_x = window_x as usize & 0x1F;
        let window_y = window_y as usize & 0x1F;
        let cell_w = 64; // TODO
        let _cell_h = 30; // TODO: PAL / screen size

        if window_left && window_top && window_x == 0 && window_y == 0 {
            return; // TODO: not exhausative, will catch most cases
        }

        // draw TO left, TO top

        if window_left && window_top && screen_y < window_y * 8 {

            let row = screen_y / 8;

            for n in 0..cell_w - window_x {
                let tile_offset = (n + (row * cell_w)) * 2;
                let tile_slice = &emu.core.mem.vdp.VRAM[nametable + tile_offset..];

                let word = (tile_slice[0] as usize) << 8 | tile_slice[1] as usize;
                let byte = word >> 8;

                let priority = (byte >> 7) & 1;

                let tile = word & 0x7FF;
                let vflip = (byte & 0x10) != 0;
                let hflip = (byte & 0x8) != 0;
                let palette = (byte & 0x60) >> 5;

                let x = n * 8;
                let y = screen_y & 7;
                let y = if vflip { y ^ 7 } else { y };
                let index = (tile * 32) + (y * 4);

                let target = if priority == 0 { &mut line_low } else { &mut line_high };

                for cursor in 0..8 {
                    let duxel = emu.core.mem.vdp.VRAM[index + (cursor / 2)];
                    let px = if cursor & 1 != 0 { duxel & 0xF } else { duxel >> 4 };

                    if px != 0 {
                        let screen_x = if hflip { cursor ^ 7 } else { cursor } + x;
                        let (r, g, b) = cram_rgb[px as usize + (palette * 0x10)];
                        let offset = screen_x * 3;
                        (*target)[offset] = r;
                        (*target)[offset + 1] = g;
                        (*target)[offset + 2] = b;
                    }
                }
            }

        }

    }
}
