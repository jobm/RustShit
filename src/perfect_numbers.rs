use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }
    match (1..num).filter(|n| num % n == 0).sum::<u64>().cmp(&num) {
        Ordering::Equal => return Some(Classification::Perfect),
        Ordering::Less => return Some(Classification::Deficient),
        Ordering::Greater => return Some(Classification::Abundant),
    }
}
// 28 -> [1, 2, 4, 7, 14]
