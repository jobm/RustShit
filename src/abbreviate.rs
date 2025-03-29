pub fn abbreviate(phrase: &str) -> String {
    let mut abbrev = String::new();
    phrase
        .to_string()
        .split(&[' ', '_', ',', '-'])
        .collect::<Vec<_>>()
        .iter()
        .for_each(|word| {
            let matched = word
                .split(char::is_lowercase)
                .filter(|c| !c.is_empty() && c.contains(char::is_alphabetic))
                .collect::<Vec<_>>();
            if matched.len() > 1 {
                abbrev.extend(matched)
            } else {
                if !word.chars().nth(0).is_none() {
                    abbrev.push(word.chars().nth(0).unwrap())
                }
            }
        });
    abbrev.to_uppercase()
}

// // assert_eq!(expected, output);
// println!("{}", abbreviate("HyperText Markup Language"));
// println!("{}", abbreviate("The Road _Not_ Taken")); //ma,m,c,k
// println!("{}", abbreviate("Something -   I made up from thin air"));
// println!("{}", abbreviate("Halley's Comet"));
// println!("{}", abbreviate("GNU Image Manipulation Program"));
// println!("{}", abbreviate("mm,kk,t,c")); //  HyperText Markup Language
// println!("{}", abbreviate("HyperText` Markup Language"));
