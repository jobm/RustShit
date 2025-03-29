pub fn encode(source: &str) -> String {
    let mut s = source.chars().collect::<Vec<_>>();
    s.dedup();
    if s.iter().to_owned().eq(&source.chars().collect::<Vec<_>>()) {
        return String::from(source);
    }

    let mut curr_count: usize = 0;
    let mut prev: char = source.chars().nth(1).unwrap();
    source
        .chars()
        .enumerate()
        .fold(String::new(), |mut acc, (i, val)| {
            if val.eq(&prev) {
                curr_count += 1;
            } else {
                acc.extend([if curr_count > 1 {
                    format!("{curr_count}{prev}")
                } else {
                    format!("{prev}")
                }]);
                prev = val;
                curr_count = 1;
            }
            if i.eq(&(source.len() - 1)) {
                acc.extend([if curr_count > 1 {
                    format!("{curr_count}{prev}")
                } else {
                    format!("{val}")
                }]);
            }
            acc
        })
}

pub fn decode(source: &str) -> String {
    let mut n: String = String::new();
    let mut res: String = String::new();
    source.chars().for_each(|c| {
        if c.is_numeric() {
            n.push(c);
        } else {
            res.extend([c
                .to_string()
                .repeat(n.to_string().parse::<usize>().unwrap_or(1))]);
            n = String::new();
        }
    });
    res
}
