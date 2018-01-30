extern crate hound;
extern crate dft;

use std::env;
use std::process;

use hound::WavReader;
use dft::{Operation, Plan, c32};

const SAMPLE_RATE: usize = 44100;

#[derive(Debug)]
struct Range(f32, f32);

fn frequency_range(mut data: Vec<c32>, bins: usize) -> Range {
    let plan = Plan::new(Operation::Forward, bins);
    dft::transform(&mut data, &plan);

    let up_to: usize = bins / 2;    // Up to Nyquist frequency
    let magnitudes: Vec<f32> = data[..up_to]
        .iter()
        .map(|&c32 { re, im }| (re.powi(2) + im.powi(2)).sqrt())
        .collect();

    let bin_size: f32 = SAMPLE_RATE as f32 / bins as f32;

    let (mut freq_bin, mut freq_mag): (usize, f32) = (0, 0.0);
    for (i, &mag) in magnitudes.iter().enumerate() {
        if mag > freq_mag {
            freq_bin = i;
            freq_mag = mag;
        }
    }

    let lower_bound = freq_bin as f32 * bin_size;
    let upper_bound = lower_bound + bin_size;

    Range(lower_bound, upper_bound)
}

fn analyze_file(filename: &str) {
    let mut reader = WavReader::open(filename).unwrap();

    let samples: Vec<f32> = reader.samples::<i32>()
        .map(|sample| sample.unwrap() as f32)
        .collect();

    const BIN_SIZE: usize = 2048;
    let data: Vec<c32> = samples[..BIN_SIZE]
        .iter()
        .map(|sample| c32::new(*sample as f32, 0.0))
        .collect();

    let freq_range = frequency_range(data, BIN_SIZE);
    println!("{:?}", freq_range);
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Requires one argument of the input filename");
        process::exit(1);
    }

    let filename = &args[1];
    analyze_file(filename);
}
