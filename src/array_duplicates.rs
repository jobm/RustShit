pub fn is_array_duplicated(list: &[u64]) -> bool {
    let mut cumulator = 0;
    for i in list {
        cumulator ^= i;
    }
    cumulator >= 1
}
