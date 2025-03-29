use std::cmp::max;

#[derive(Debug)]
pub struct Item {
    pub weight: u32,
    pub value: u32,
}

pub fn maximum_value(max_weight: u32, items: &[Item]) -> u32 {
    let mut bag = vec![vec![0; (max_weight + 1) as usize]; items.len() + 1];
    for row in 0..=items.len() {
        for w in 0..=max_weight {
            if row == 0 || w == 0 {
                bag[row][w as usize] = 0
            } else if items[row - 1].weight <= w {
                bag[row][w as usize] = max(
                    items[row - 1].value + bag[row - 1][(w - items[row - 1].weight) as usize],
                    bag[row - 1][w as usize],
                )
            } else {
                bag[row][w as usize] = bag[row - 1][w as usize]
            }
        }
    }
    bag[items.len()][max_weight as usize]
}

pub fn maximum_value_dp(max_weight: u32, items: &[Item]) -> u32 {
    let mut solutions = vec![0; (max_weight + 1) as usize];
    for i in 1..=items.len() {
        for w in (1..=max_weight).rev() {
            if items[i - 1].weight <= w {
                solutions[w as usize] = max(
                    solutions[w as usize],
                    solutions[(w - items[i - 1].weight) as usize] + items[i - 1].value,
                )
            }
        }
    }
    solutions[max_weight as usize]
}
