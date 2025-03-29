use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let mut res = nucleotide_counts(dna)?;
    res.remove(&nucleotide).ok_or(nucleotide)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut count: HashMap<char, usize> = HashMap::from([('A', 0), ('T', 0), ('C', 0), ('G', 0)]);
    for c in dna.chars() {
        match count.get_mut(&c) {
            Some(count) => *count += 1,
            None => return Err(c),
        }
    }
    Ok(count)
}
