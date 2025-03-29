use std::collections::HashMap;

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    if input.eq(&vec![vec![]]) {
        return vec![];
    }
    let mut row = 0;
    let mut points: Vec<(usize, usize)> = Vec::new();
    let mut min_cache: HashMap<(usize, usize), u64> = HashMap::new();
    while row < input.len() {
        let mut col = 0;
        let max_ = input[row].iter().max().unwrap();
        while col < input[0].len() {
            let mut tempr = row;
            let mut min_ = input[tempr][col];
            while tempr < input.len() {
                let cached_min = min_cache.get(&(tempr, col));
                if cached_min.is_some() {
                    min_ = *cached_min.unwrap();
                } else {
                    min_ = min_.min(input[tempr][col]);
                    min_cache.insert((tempr, col), min_);
                }
                tempr += 1;
            }
            if min_.eq(max_) {
                points.push((row, col));
            }
            col += 1;
        }
        row += 1;
    }
    points
}
