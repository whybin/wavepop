use hound::WavReader;
use rustfft::FFTplanner;
use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;

fn fft_to_freq(bins: &Vec<Complex<f32>>, sample_rate: usize) -> usize {
    let up_to: usize = bins.len() / 2;    // Up to Nyquist frequency
    let magnitudes: Vec<f32> = bins[..up_to]
        .iter()
        .map(|&Complex { re, im }| (re.powi(2) + im.powi(2)).sqrt())
        .collect();

    let bin_size: usize = sample_rate / bins.len();

    let (mut freq_bin, mut freq_mag): (usize, f32) = (0, 0.0);
    for (i, &mag) in magnitudes.iter().enumerate() {
        if mag > freq_mag {
            freq_bin = i;
            freq_mag = mag;
        }
    }

    freq_bin * bin_size as usize
}

fn get_frequencies(
    samples: &Vec<f32>, sample_rate: usize, num_points: usize, bps: usize
    ) -> Vec<usize> {
    let samples_per: usize = sample_rate as usize / bps;
    let num_freq: usize = samples.len() / samples_per;
    let mut frequencies: Vec<usize> = vec![0; num_freq];

    let mut planner = FFTplanner::new(false);
    let fft = planner.plan_fft(num_points);

    for i in 0..num_freq {
        let offset = i * samples_per;
        // Further slice the sample size by num_points
        let mut input: Vec<Complex<f32>> = samples[offset..offset + num_points]
            .iter()
            .map(|&sample| Complex::new(sample as f32, 0.0))
            .collect();

        let mut output = vec![Complex::zero(); num_points];
        fft.process(&mut input, &mut output);

        frequencies[i] = fft_to_freq(&output, sample_rate);
    }

    frequencies
}

pub fn analyze_file(filename: &str, num_seconds: usize, bps: usize) -> Vec<usize> {
    let mut reader = WavReader::open(filename).unwrap();
    let sample_rate: usize = reader.spec().sample_rate as usize;

    let samples: Vec<f32> = reader.samples::<i32>()
        .take(num_seconds * sample_rate)
        .map(|sample| sample.unwrap() as f32)
        .collect();

    let num_points: usize = 4096;

    let mut frequencies = get_frequencies(&samples, sample_rate, num_points, bps);
    frequencies.retain(|&freq| freq < 20000);

    frequencies
}
