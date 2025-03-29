pub fn number(user_number: &str) -> Option<String> {
    fn rm_code(s: &str) -> (bool, &str) {
        if s.starts_with("1") {
            return (true, s.strip_prefix("1").unwrap());
        } else if s.starts_with("+1") {
            return (true, s.strip_prefix("+1").unwrap());
        } else {
            (false, s)
        }
    }
    let has_code = rm_code(user_number).0;
    let res: String = rm_code(user_number)
        .1
        .chars()
        .filter(|c| c.is_numeric())
        .collect();
    if !has_code && res.len() > 10 {
        return None;
    };
    if res.len() < 10
        || res.starts_with(&['0', '1'])
        || res.chars().nth(3).eq(&Some('0'))
        || res.chars().nth(3).eq(&Some('1'))
    {
        return None;
    }
    Some(res)
}
