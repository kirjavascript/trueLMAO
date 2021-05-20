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
        cell_w: usize,
        cell_h: usize,
        screen_y: usize,
        screen_width: usize,
        nametable: usize,
        hscroll: usize,
        vscroll_offset: usize,
        layer_priority: usize,
    ) {
        for screen_x in 0..screen_width {
            // TODO: perf optim by doing things in tiles instead of pixels
            let vscroll = emu.core.mem.vdp.vscroll(screen_x)[vscroll_offset] as usize;

            let plane_width = cell_w * 8;
            let plane_height = cell_h * 8;

            let hscroll_rem = hscroll % plane_width;
            let x_offset = (screen_x + plane_width - hscroll_rem) % plane_width;
            let y_offset = (screen_y + vscroll) % plane_height;

            let tile_index = ((x_offset / 8) + (y_offset / 8 * cell_w)) * 2;
            let tile_slice = &emu.core.mem.vdp.VRAM[nametable + tile_index..];

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

            let hline = if hflip { x_offset ^ 0xF } else { x_offset };
            let x_offset = (hline & 6) >> 1;
            let vline = y_offset & 7;
            let y_offset = if vflip { vline ^ 7 } else { vline } * 4;

            let px = emu.core.mem.vdp.VRAM[(tile * 32) + x_offset + y_offset];
            let px = if hline & 1 == 0 { px >> 4 } else { px & 0xF };

            if px != 0 {
                let (r, g, b) = emu.core.mem.vdp.color(palette, px as _);

                let screen_offset = (screen_x + (screen_y * screen_width)) * 3;

                emu.gfx.screen[screen_offset] = r;
                emu.gfx.screen[screen_offset + 1] = g;
                emu.gfx.screen[screen_offset + 2] = b;
            }
        }
    }

    pub fn draw_sprite_line(
        emu: &mut Megadrive,
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

                if x_offset >= 0 && x_offset <= screen_width as isize {

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
                        let (r, g, b) = emu.core.mem.vdp.color(sprite.palette, px as _);

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
        screen_y: usize,
        screen_width: usize,
        priority: usize,
    ) {

    }
}
