/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let encoder = "abcdefghijklmnopqrstuvwxyz";
    compute(plain, encoder, true)
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    let decoder = "zyxwvutsrqponmlkjihgfedcba";
    compute(cipher, decoder, false)
}

pub fn compute(word: &str, enc_dec: &str, ws: bool) -> String {
    word.lines()
        .map(|w| {
            let word = w
                .chars()
                .map(|letter| {
                    if let Some(idx) = enc_dec
                        .chars()
                        .position(|x| x == letter.to_ascii_lowercase())
                    {
                        enc_dec.chars().nth((enc_dec.len() - idx) - 1).unwrap()
                    } else {
                        letter
                    }
                })
                .collect::<String>();
            match ws {
                true => word
                    .chars()
                    .filter(|c| !c.is_ascii_whitespace() && !c.is_ascii_punctuation())
                    .collect::<Vec<_>>()
                    .chunks(5)
                    .map(|c| c.iter().collect::<String>())
                    .collect::<Vec<_>>()
                    .join(" "),
                false => word.chars().filter(|c| !c.is_ascii_whitespace()).collect(),
            }
        })
        .collect()
}
