extern crate nsvg;

use std::cmp::max;

use rustsvg::node::element::path::Data;
use rustsvg::node::element::{Definitions, Path, LinearGradient, Stop};
use rustsvg::{Document, save};
use image;

use chunker::PatternMap;

pub fn to_svg_image(
    pattern_map: &PatternMap, hor_spacing: usize, height: usize
    ) -> image::RgbaImage {
    let ver_spacing = max(1, height / pattern_map.num_patterns());

    let mut def = Definitions::new();

    for i in 0..pattern_map.num_patterns() {
        let colors = &pattern_map.pattern_at(i as u32).colors;

        let stop0 = Stop::new()
            .set("offset", "0%")
            .set("stop-color", &colors.0[..]);
        let stop1 = Stop::new()
            .set("offset", "100%")
            .set("stop-color", &colors.1[..]);

        let grad = LinearGradient::new()
            .set("id", format!("gradient{}", i))
            .add(stop0)
            .add(stop1);
        def = def.add(grad);
    }

    let mut doc = Document::new()
        .add(def);

    // Iterate over patterns in order
    for i in 0..pattern_map.num_indices() {
        // For each instance of the pattern
        for &pattern_idx in pattern_map.pattern_indices(i) {
            let pattern = pattern_map.pattern_at(pattern_idx);
            let len = pattern.length;

            let data = Data::new()
                .move_to((i * hor_spacing,
                          pattern_idx as usize * ver_spacing))
                .line_by((len as usize * hor_spacing, 0));

            let path = Path::new()
                .set("fill", "none")
                .set("stroke", format!("url(#gradient{})", pattern_idx))
                .set("stroke-width", ver_spacing)
                .set("d", data);

            doc = doc.add(path);
        }
    }

    let filepath = "assets/images/output.svg";
    save(filepath, &doc).unwrap();

    // Convert to PNG
    let svg = nsvg::parse_file(filepath, "px", 100.0);
    let image = nsvg::rasterize(svg, 1.0);

    image
}
