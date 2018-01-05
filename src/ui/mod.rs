extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::ttf;
use std::time::Duration;
use std::path::Path;

static FONT_PATH: &'static str = "assets/font.ttf";

pub struct UI {
    ctx: sdl2::Sdl,
    video: sdl2::VideoSubsystem,
    debug: sdl2::render::Canvas<sdl2::video::Window>,
    // ttf: ttf::Sdl2TtfContext,
    // font: Option<ttf::Font<'ttf, 'static>>,
}

// fn get_font<'ttf, 'r>() -> sdl2::ttf::Font<'ttf, 'r> {
//     let ttf_context = ttf::init().unwrap();
//     ttf_context.load_font(FONT_PATH, 8).unwrap()
// }

impl UI {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        // debug
        let debug_window = video_subsystem.window("trueLMAO", 640, 480)
            .position_centered().build().unwrap();
        let mut debug_canvas = debug_window.into_canvas()
            .accelerated().build().unwrap();

        debug_canvas.set_draw_color(Color::RGBA(0,0,0,255));
        debug_canvas.clear();

        // font
        let ttf_context = ttf::init().unwrap();
        let font = ttf_context.load_font(FONT_PATH, 8).unwrap();

    let texture_creator = debug_canvas.texture_creator();
    let surface = font.render("testing")
        .blended_wrapped(Color::RGBA(255, 255, 255, 255), 1).unwrap();
    let rect = Rect::new(0, 0, 400, 120);
    let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
    debug_canvas.copy(&texture, None, Some(rect));

        debug_canvas.present();

        UI {
            ctx: sdl_context,
            video: video_subsystem,
            debug: debug_canvas,
            // ttf: ttf_context,
            // font: Some(font),
        }

    }

    pub fn render(&self) -> bool {

        let mut running = true;
        let mut event_pump = self.ctx.event_pump().unwrap();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    running = false;
                },
                q => {
                    // println!("{:#?}", q);
                },
            }
        }

        running
    }
}

pub fn init() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("trueLMAO", 640, 480)
        .position_centered().build().unwrap();

    let mut canvas = window.into_canvas()
        .accelerated().build().unwrap();

    let texture_creator = canvas.texture_creator();

    canvas.set_draw_color(Color::RGBA(0,0,0,255));
    canvas.clear();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let font_path: &Path = Path::new("assets/font.ttf");
    let ttf_context = sdl2::ttf::init().unwrap();
    let mut font = ttf_context.load_font(font_path, 8).unwrap();
    let surface = font.render("bonk")
        .blended_wrapped(Color::RGBA(255, 255, 255, 255), 1).unwrap();
    let rect = Rect::new(0, 0, 400, 30);
    let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
    canvas.copy(&texture, None, Some(rect));

    canvas.present();

    let mut i: f32 = 0.0;

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    break 'main;
                },
                q => {
                    // println!("{:#?}", q);
                },
            }
        }

        canvas.clear();

        let shift = (i.sin() * 100.0).abs() as u32;

    let rect = Rect::new((shift/2) as i32, (100-shift) as i32, 400 - shift, 30 + shift );
    canvas.copy(&texture, None, Some(rect));

    canvas.present();

        i += 0.001;
    }
    // std::thread::sleep(Duration::from_millis(100));
}
