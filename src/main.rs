extern crate wavepop;

use std::env;
use std::process;

use wavepop::frequency;
use wavepop::chunker;
use wavepop::svg;
use wavepop::sound;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Requires one argument of the input filename");
        process::exit(1);
    }

    let filename = &args[1];
    let data: Vec<_> = frequency::analyze_file(filename);

    let pattern_map = chunker::chunk(&data);
    svg::to_svg(&pattern_map, 600, 600);

    let frequencies: Vec<u32> = data
        .iter()
        .map(|&freq| freq as u32)
        .collect();
    let sink = sound::compose(&frequencies, 500);
    sound::play(&sink);
}
