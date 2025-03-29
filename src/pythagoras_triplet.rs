use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut set: HashSet<[u32; 3]> = HashSet::new();
    for a in 1..sum / 3 {
        for b in a + 1..sum / 2 {
            let c = sum - a - b;
            if a * a + b * b == c * c {
                let mut triplet = [a, b, c];
                triplet.sort(); // Sort to avoid duplicates
                set.insert(triplet);
            }
        }
    }
    set
}
