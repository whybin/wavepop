use suffix::SuffixTable;

pub fn chunk(data: &Vec<usize>) {
    let bytes: Vec<u16> = data
        .iter()
        .map(|&datum| datum as u16)
        .collect();
    let text = String::from_utf16(&bytes).unwrap();

    let table = SuffixTable::new(text);
    println!("{:?}", &table);
    println!("{:?}", &table.table());
    println!("{:?}", &table.lcp_lens());
}
