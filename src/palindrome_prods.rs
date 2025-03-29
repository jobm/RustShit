/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        if is_palindrome(value) {
            Some(Palindrome(value))
        } else {
            None
        }
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
fn is_palindrome(n: u64) -> bool {
    n.to_string().chars().rev().eq(n.to_string().chars())
}
pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut prods = vec![];
    for n in min..max + 1 {
        let prod = find_prods(n, max);
        prods.extend(prod);
    }
    prods.sort();
    prods.dedup();
    if prods.first().is_none() && prods.last().is_none() {
        return None;
    }
    let min = Palindrome::new(*prods.first().unwrap());
    let max = Palindrome::new(*prods.last().unwrap());
    Some((min.unwrap(), max.unwrap()))
}

fn find_prods(n: u64, max: u64) -> Vec<u64> {
    let n_prods = vec![];
    if max.eq(&0) {
        return n_prods;
    }
    let mut n_prods = find_prods(n, max - 1);
    if is_palindrome(n * max) {
        n_prods.push(n * max);
    }
    n_prods
}

// fn prod_in_map(a: u64, b: u64) -> (u64, bool) {
//     let mut map = HashMap::<(u64, u64), u64>::new();
//     let p = map.get(&(a, b));
//     if p.is_some() {
//         return (*p.unwrap(), true);
//     } else {
//         *map.get_mut(&(a, b)).unwrap() = a * b;
//     }
//     (0, false)
// }
