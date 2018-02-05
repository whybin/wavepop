extern crate piston_window;
extern crate gfx_device_gl;
extern crate graphics;
extern crate image;

extern crate wavepop;

use std::env;
use std::process;

use piston_window::*;
use gfx_device_gl::Resources;

use wavepop::frequency;
use wavepop::chunker;
use wavepop::svg;
use wavepop::display::*;
use wavepop::sound;

const NUM_SECONDS: usize = 90;
const WIN_WIDTH: usize = 800;
const WIN_HEIGHT: usize = 800;
const HOR_SPACING: usize = 18;
const BPS: usize = 2;

struct VisualFrame {
    window: PistonWindow,
    texture: Texture<Resources>,
    offset: f64
}

impl VisualFrame {
    fn new(width: usize, height: usize, image: &image::RgbaImage)
        -> VisualFrame {
        let mut window = new_window(width as u32, height as u32);
        let texture = new_texture(&mut window, &image);

        VisualFrame { window, texture, offset: 0.0 }
    }
}

impl WindowFrame for VisualFrame {
    fn handle_event(&mut self) -> bool {
        if let Some(event) = self.window.next() {
            let &mut VisualFrame { offset, ref texture, .. } = self;

            self.window.draw_2d(&event, |ctx, gl| {
                graphics::clear(color::grey(0.1), gl);

                let transform = ctx.transform
                    .trans(offset, 0.0);

                graphics::image(texture, transform, gl);
            });

            if let Some(args) = event.update_args() {
                self.offset -= args.dt * (HOR_SPACING * BPS) as f64;
            }

            true
        } else {
            false
        }
    }
}

fn handle_file(filename: &str) {
    let data: Vec<_> = frequency::analyze_file(filename, NUM_SECONDS, BPS);

    let pattern_map = chunker::chunk(&data);
    let image = svg::to_svg_image(&pattern_map, HOR_SPACING, WIN_HEIGHT);

    let mut visual_frame = VisualFrame::new(WIN_WIDTH, WIN_HEIGHT, &image);

    // Start sound
    let frequencies: Vec<u32> = data
        .iter()
        .map(|&freq| freq as u32)
        .collect();
    // Must keep variable around
    let _sink = sound::compose(&frequencies, 1000 / BPS as u64);

    // Start processing window events
    while visual_frame.handle_event() { }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Requires one argument of the input filename");
        process::exit(1);
    }

    handle_file(&args[1]);
}
