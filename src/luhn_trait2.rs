pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

/// Here is the example of how to implement custom Luhn trait
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T> Luhn for T
where
    T: ToString,
{
    fn valid_luhn(&self) -> bool {
        let code = self.to_string().clone();
        if code.trim() == "0"
            || code.contains(char::is_alphabetic)
            || code.chars().any(|c| c.is_ascii_punctuation())
        {
            return false;
        }
        code.chars()
            .filter(|num| num.is_ascii_digit())
            .rev()
            .enumerate()
            .map(|(idx, num)| {
                if idx % 2 == 1 {
                    if (num.to_digit(10).unwrap() * 2) > 9 {
                        num.to_digit(10).unwrap() * 2 - 9
                    } else {
                        num.to_digit(10).unwrap() * 2
                    }
                } else {
                    num.to_digit(10).unwrap()
                }
            })
            .sum::<u32>()
            .rem_euclid(10)
            == 0
    }
}
