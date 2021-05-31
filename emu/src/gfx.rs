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

    pub fn draw_plane_line(
        emu: &mut Megadrive,
        cram_rgb: &[(u8, u8, u8); 64],
        cell_w: usize,
        cell_h: usize,
        screen_y: usize,
        screen_width: usize,
        nametable: usize,
        hscroll: usize,
        vscroll_offset: usize,
        layer_priority: usize,
    ) {
        let columns = emu.core.mem.vdp.registers[0xB] & 4 != 0;
        let columns = true;

        let vscroll = emu.core.mem.vdp.vscroll(0)[vscroll_offset] as usize;
        // TODO: 16px columns rather than fullscreen

        let plane_width = cell_w * 8;
        let plane_height = cell_h * 8;

        let mut screen_x = 0;

        while screen_x < screen_width {
            let mut width = 8;

            if screen_x == 0 {
                let hoff = hscroll % 8;
                if hoff > 0 {
                    width = hoff;
                }
            }

            let hscroll_rem = hscroll % plane_width;
            let x_offset = (screen_x + plane_width - hscroll_rem) % plane_width;
            let y_offset = (screen_y + vscroll) % plane_height;

            let tile_index = ((x_offset / 8) + (y_offset / 8 * cell_w)) * 2;

            let byte = emu.core.mem.vdp.VRAM[nametable + tile_index] as usize;
            let next = emu.core.mem.vdp.VRAM[nametable + tile_index + 1] as usize;
            let word = byte << 8 | next;

            let priority = (byte >> 7) & 1;

            if priority == layer_priority {
                let tile = word & 0x7FF;
                let vflip = (byte & 0x10) != 0;
                let hflip = (byte & 0x8) != 0;
                let palette = (byte & 0x60) >> 5;

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

                let start = 8 - width;
                let end = 8.min(screen_width - screen_x);

                for (x, px) in (&pixels[start..end]).iter().enumerate() {
                    if *px != 0 {
                        let screen_x = screen_x + x;
                        let (r, g, b) = cram_rgb[*px as usize + (palette * 0x10)];
                        let screen_offset = (screen_x + (screen_y * screen_width)) * 3;
                        emu.gfx.screen[screen_offset] = r;
                        emu.gfx.screen[screen_offset + 1] = g;
                        emu.gfx.screen[screen_offset + 2] = b;
                    }
                }
            }

            screen_x += width;
        };


    }

    pub fn draw_sprite_line(
        emu: &mut Megadrive,
        cram_rgb: &[(u8, u8, u8); 64],
        sprites: &Vec<crate::vdp::Sprite>,
        screen_y: usize,
        screen_width: usize,
        priority: usize,
    ) {
        for sprite in sprites.iter().rev() {
            if sprite.priority != priority {
                continue
            }
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
                        let screen_offset = (x_offset as usize + (screen_y * screen_width)) * 3;

                        if screen_offset + 2 <= emu.gfx.screen.len() {
                            emu.gfx.screen[screen_offset] = r;
                            emu.gfx.screen[screen_offset + 1] = g;
                            emu.gfx.screen[screen_offset + 2] = b;
                        }
                    }
                }
            }
        }
    }


    pub fn draw_window_line(
        emu: &mut Megadrive,
        cram_rgb: &[(u8, u8, u8); 64],
        screen_y: usize,
        screen_width: usize,
        layer_priority: usize,
    ) {
        // TODO: support non-320 size nametable
        // TODO: plane A / window exclusivity (perf)
        // TODO: cache these variables
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

                if priority != layer_priority {
                    continue
                }

                let tile = word & 0x7FF;
                let vflip = (byte & 0x10) != 0;
                let hflip = (byte & 0x8) != 0;
                let palette = (byte & 0x60) >> 5;

                let x = n * 8;
                let y = screen_y & 7;
                let y = if vflip { y ^ 7 } else { y };
                let index = (tile * 32) + (y * 4);

                for cursor in 0..8 {
                    let duxel = emu.core.mem.vdp.VRAM[index + (cursor / 2)];
                    let px = if cursor & 1 != 0 { duxel & 0xF } else { duxel >> 4 };

                    if px != 0 {
                        let screen_x = if hflip { cursor ^ 7 } else { cursor } + x;
                        let (r, g, b) = cram_rgb[px as usize + (palette * 0x10)];
                        let screen_offset = (screen_x + (screen_y * screen_width)) * 3;
                        emu.gfx.screen[screen_offset] = r;
                        emu.gfx.screen[screen_offset + 1] = g;
                        emu.gfx.screen[screen_offset + 2] = b;
                    }
                }
            }

        }

    }
}
