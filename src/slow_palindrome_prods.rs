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
    let mut mn = None;
    let mut mx = None;
    for i in min..=max {
        if i % 10 == 0 {
            continue;
        }
        for j in i..=max {
            if j % 10 == 0 {
                continue;
            }
            if is_palindrome(i * j) {
                if (i * j) > mx.unwrap_or(u64::MIN) {
                    mx = Some(i * j);
                }
                if (i * j) < mn.unwrap_or(u64::MAX) {
                    mn = Some(i * j);
                }
            }
        }
    }
    match (mn, mx) {
        (Some(min), Some(max)) => {
            Some((Palindrome::new(min).unwrap(), Palindrome::new(max).unwrap()))
        }
        _ => None,
    }
}
