pub fn find_permutations(v: &mut [u32]) -> Vec<Vec<u32>> {
    let max = v.len() - 1;
    let mut prods: Vec<Vec<u32>> = vec![];
    permutate(&mut prods, v.as_mut(), 0, max);
    prods
}

fn permutate(prods: &mut Vec<Vec<u32>>, v: &mut [u32], min: usize, max: usize) {
    if min == max {
        prods.push(v.to_vec());
        return;
    }
    for i in min..=max {
        v.swap(min, i);
        permutate(prods, v, min + 1, max);
        v.swap(min, i);
    }
}
