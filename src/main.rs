extern crate piston_window;
extern crate graphics;
extern crate wavepop;

use std::env;
use std::process;

use piston_window::*;

use wavepop::frequency;
use wavepop::chunker;
use wavepop::svg;
use wavepop::display;
use wavepop::sound;

const WIN_WIDTH: usize = 800;
const WIN_HEIGHT: usize = 800;
const HOR_SPACING: usize = 18;

#[allow(unreachable_code)]
fn handle_file(filename: &str) {
    let data: Vec<_> = frequency::analyze_file(filename);

    let pattern_map = chunker::chunk(&data);
    let image = svg::to_svg_image(&pattern_map, HOR_SPACING, WIN_HEIGHT);

    let mut window = display::new_window(WIN_WIDTH as u32, WIN_HEIGHT as u32);
    let texture = display::new_texture(&mut window, &image);

    while let Some(event) = window.next() {
        window.draw_2d(&event, |ctx, gl| {
            graphics::clear(color::grey(0.1), gl);
            graphics::image(&texture, ctx.transform, gl);
        });
    }
    return;

    let frequencies: Vec<u32> = data
        .iter()
        .map(|&freq| freq as u32)
        .collect();
    let sink = sound::compose(&frequencies, 500);
    sound::play(&sink);
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Requires one argument of the input filename");
        process::exit(1);
    }

    handle_file(&args[1]);
}
