extern crate piston_window;
extern crate wavepop;

use std::env;
use std::process;

use piston_window::*;

use wavepop::frequency;
use wavepop::chunker;
use wavepop::svg;
use wavepop::display;
use wavepop::sound;

#[allow(unreachable_code)]
fn handle_file(filename: &str) {
    let data: Vec<_> = frequency::analyze_file(filename);

    let pattern_map = chunker::chunk(&data);
    let width = 800;
    let height = 800;
    let image = svg::to_svg_image(&pattern_map, width, height);

    let mut window = display::new_window(width as u32, height as u32);
    let texture = display::new_texture(&mut window, &image);

    while let Some(event) = window.next() {
        window.draw_2d(&event, |ctx, gl| {
            clear(color::BLACK, gl);
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
