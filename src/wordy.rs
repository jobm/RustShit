pub fn answer(command: &str) -> Option<i32> {
    fn digits(s: &str) -> Vec<i32> {
        s.chars()
            .filter(|c| c.is_numeric())
            .map(|c| c.to_digit(10).unwrap_or(0) as i32)
            .collect::<Vec<i32>>()
    }
    match command {
        _ if command.contains("plus") => Some(digits(command).iter().sum::<i32>()),
        _ if command.contains("minus") => {
            let max: i32 = *digits(command).iter().max().unwrap();
            let res = digits(command).iter().fold(max, |mut ans, it| {
                ans -= it;
                ans
            });
            Some(res)
        }
        _ if command.contains("multiplied") => {
            let res = digits(command).iter().fold(1, |mut ans, it| {
                ans *= it;
                ans
            });
            Some(res)
        }
        _ if command.contains("divided") => {
            let res = digits(command).iter().fold(1, |mut ans, it| {
                ans /= it;
                ans
            });
            Some(res)
        }
        _ => Some(
            *command
                .chars()
                .filter(|c| c.is_numeric())
                .map(|c| c.to_digit(10).unwrap_or(0) as i32)
                .collect::<Vec<_>>()
                .first()
                .unwrap(),
        ),
    }
}
