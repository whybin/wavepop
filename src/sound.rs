use std::time::Duration;

#[allow(deprecated)]
use rodio::{get_default_endpoint, Sink, source, Source};

#[allow(deprecated)]
fn new_sink() -> Sink {
    let endpoint = get_default_endpoint().unwrap();

    Sink::new(&endpoint)
}

pub fn compose(frequencies: &Vec<u32>, millis_per: u64) -> Sink {
    let sink = new_sink();

    for &freq in frequencies.iter() {
        let src = source::SineWave::new(freq);
        let src = src.take_duration(Duration::from_millis(millis_per));
        sink.append(src);
    }

    sink
}

pub fn play(sink: &Sink) {
    sink.play();
    sink.sleep_until_end();
}
