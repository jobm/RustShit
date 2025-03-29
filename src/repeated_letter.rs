// pub fn first_repeated(s: &str) -> Option<char> {
//     // 'wewe' -> w
//     let mut prev = 0;
//     for c in s[1..].chars() {
//         if c.eq(&s.chars().nth(prev).unwrap()) {
//             return Some(c);
//         }
//         prev += 1;
//     }
//     None
// }

pub fn first_repeated(s: &str) -> Option<char> {
    // find first repeated letters .e.g;
    // "wewe" -> None, "wewerr" -> Some("rr"),  "txtffn" => Some("ff")
    s.chars()
        .zip(s.chars().skip(1))
        .find(|&(x, y)| x == y)
        .map(|(c, _)| c)
}
