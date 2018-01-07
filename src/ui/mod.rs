extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::ttf;
use std::time::Duration;
use std::path::Path;

static FONT_PATH: &'static str = "assets/font.ttf";

pub struct UI<'ttf, 'r> {
    ctx: sdl2::Sdl,
    video: sdl2::VideoSubsystem,
    font: ttf::Font<'ttf, 'r>,
    debug: sdl2::render::Canvas<sdl2::video::Window>,
}

impl<'ttf, 'r> UI<'ttf, 'r> {
    pub fn new(ttf_context: &'ttf ttf::Sdl2TtfContext) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        // debug
        let debug_window = video_subsystem.window("trueLMAO", 640, 480)
            .position_centered().build().unwrap();
        let mut debug_canvas = debug_window.into_canvas()
            .accelerated().build().unwrap();

        debug_canvas.set_draw_color(Color::RGBA(0,0,0,255));
        debug_canvas.clear();
        debug_canvas.present();

        // font
        let font = ttf_context.load_font(FONT_PATH, 8).unwrap();

        UI {
            ctx: sdl_context,
            video: video_subsystem,
            debug: debug_canvas,
            font: font,
        }

    }

    pub fn render(&mut self, i: u32) -> bool {

        let mut running = true;
        let mut events = self.ctx.event_pump().unwrap();

        for event in events.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    running = false;
                },
                _ => {},
            }
        }

        self.click(|x, y| {
            println!("{} {}", x, y);
        });

    let surface = self.font.render(format!("{}", i).as_ref())
        .blended_wrapped(Color::RGBA(255, 255, 255, 255), 1).unwrap();
    let rect = Rect::new(0, 0, 200, 40);

    let texture_creator = self.debug.texture_creator();
    let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
    self.debug.clear();
    self.debug.copy(&texture, None, Some(rect));
    self.debug.present();

        running
    }

    fn click(&self, func: fn(u32, u32)) {
    }
    fn draw_text(&self) {
        // remove bg
    }
}
