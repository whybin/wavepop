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

pub fn chunk(data: &Vec<usize>) {
    let bytes: Vec<u8> = normalize(&data);
    let text = String::from_utf8(bytes).unwrap();

    let table = SuffixTable::new(text);
    println!("{:?}", &table);
    println!("{:?}", &table.table());
    println!("{:?}", &table.lcp_lens());
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
}
