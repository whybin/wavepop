use std::cmp::max;

use rustsvg::node::element::path::Data;
use rustsvg::node::element::Path;
use rustsvg::{Document, save};

use chunker::PatternMap;

pub fn to_svg(pattern_map: &PatternMap, width: usize, height: usize) {
    let hor_spacing = max(1, width / pattern_map.num_indices());
    let ver_spacing = max(1, height / pattern_map.num_patterns());
    const HOR_UNITS: u32 = 6;
    
    let mut doc = Document::new()
        .set("viewbox", (0, 0, hor_spacing, ver_spacing));

    for i in 0..pattern_map.num_indices() {
        for &pattern_idx in pattern_map.patterns_at(i) {
            let len = pattern_map.pattern_length_at(pattern_idx);
            let data = Data::new()
                .move_to((i as usize * hor_spacing,
                          pattern_idx as usize * ver_spacing))
                .line_by((len * HOR_UNITS, 0));

            let path = Path::new()
                .set("fill", "none")
                .set("stroke", "gray")
                .set("stroke-width", 3)
                .set("d", data);

            doc = doc.add(path);
        }
    }

    save("assets/images/output.svg", &doc).unwrap();
}
