use suffix::SuffixTable;

fn normalize(data: &Vec<usize>) -> Vec<u8> {
    let mut unique: Vec<usize> = Vec::new();
    data.iter()
        .for_each(|&datum| {
            if unique.contains(&datum) == false {
                unique.push(datum);
            }
        });

    if unique.len() > 256 {
        panic!("Only normalize data with up to 256 unique values");
    }

    let ret: Vec<u8> = data
        .iter()
        .map(|&datum| unique.iter().position(|&x| x == datum).unwrap() as u8)
        .collect();

    ret
}

struct PatternMap {
    /// Array of lengths
    patterns: Vec<u32>,
    /// For each index, an array of pattern indices
    order: Vec<Vec<u32>>
}

impl PatternMap {
    fn new(num_indices: usize) -> PatternMap {
        PatternMap {
            patterns: vec![],
            order: vec![vec![]; num_indices]
        }
    }

    fn patterns_at(&self, index: usize) -> &Vec<u32> {
        &self.order[index]
    }

    /// Append pattern length and retrieve index
    fn new_pattern(&mut self, length: u32) -> u32 {
        self.patterns.push(length);

        self.patterns.len() as u32 - 1
    }

    fn use_pattern(&mut self, index: u32, pattern_index: u32) {
        self.order[index as usize].push(pattern_index);
    }
}

/// Items 1..n of `lcp` must contain values greater than 1
fn add_patterns(pattern_map: &mut PatternMap, indices: &[u32], lcp: &[u32]) {
    for i in 1..lcp.len() {
        let len = lcp[i];
        let pattern_index = pattern_map.new_pattern(len);

        // Look ahead
        for j in i..lcp.len() {
            if lcp[j] < len {
                break;
            }

            pattern_map.use_pattern(indices[j], pattern_index);
        }

        // Look back
        let mut k = 0;
        for j in (0..i).rev() {
            if lcp[j] < len {
                k = j;
                break;
            }

            pattern_map.use_pattern(indices[j], pattern_index);
        }

        pattern_map.use_pattern(indices[k], pattern_index);
    }
}

/// `lcp` always starts with the value 0
fn patternize(indices: &[u32], lcp: &[u32]) -> PatternMap {
    let mut i = 1;
    let mut pattern_map = PatternMap::new(lcp.len());

    loop {
        let mut it = lcp[i..].iter();
        if let Some(start) = it.position(|&prefix_len| prefix_len > 1) {
            let end = match it.position(|&prefix_len| prefix_len <= 1) {
                Some(idx) => idx,
                None => lcp.len()
            };

            add_patterns(
                &mut pattern_map,
                &indices[start - 1..end],
                &lcp[start - 1..end]
                );
            i = end;
        } else {
            break;
        }
    }

    pattern_map
}

fn patternize_text(text: String) {
    let table = SuffixTable::new(&text[..]);

    patternize(&table.table(), &table.lcp_lens());
}

pub fn chunk(data: &Vec<usize>) {
    let bytes: Vec<u8> = normalize(&data);
    let text = String::from_utf8(bytes).unwrap();

    patternize_text(text);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "unique values")]
    fn normalize_panics_if_too_many_unique_values() {
        let num = 257;
        let mut v: Vec<usize> = vec![0; num];

        for i in 0..num {
            v[i] = i;
        }

        normalize(&v);
    }

    #[test]
    fn normalize_empty_to_empty() {
        let empty: Vec<usize> = vec![];
        assert_eq!(normalize(&empty), vec![]);
    }

    #[test]
    fn normalize_single_value_to_zero() {
        let v: Vec<usize> = vec![42];
        assert_eq!(normalize(&v), vec![0]);
    }

    #[test]
    fn normalize_varying_values() {
        let v: Vec<usize> = vec![42, 3, 2, 42, 2];
        let e: Vec<u8> = vec![0, 1, 2, 0, 2];
        assert_eq!(normalize(&v), e);
    }

    #[test]
    fn patternize_one_item_produces_no_patterns() {
        let indices = vec![0];
        let lcp = vec![0];
        let map = patternize(&indices, &lcp);
        assert_eq!(map.patterns_at(0).len(), 0);
    }
}
