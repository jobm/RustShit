#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

const ALPHA: &str = "abcdefghijklmnopqrstuvwxyz";

pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    let (gcd, _, _) = egcd(a, ALPHA.len() as i32);
    if gcd != 1 {
        return Err(AffineCipherError::NotCoprime(a));
    }
    Ok(plaintext
        .to_lowercase()
        .chars()
        .filter(|c| !c.is_ascii_punctuation() && !c.is_whitespace())
        .map(|c| {
            let key = a * ALPHA.find(c).unwrap_or_default() as i32;
            match c.is_alphabetic() {
                true => Some(
                    ALPHA
                        .chars()
                        .nth((key + b).rem_euclid(ALPHA.len() as i32) as usize)
                        .unwrap_or_default(),
                ),
                false => Some(c),
            }
        })
        .collect::<Vec<_>>()
        .chunks(5)
        .map(|c| c.iter().map(|c| c.unwrap()).collect::<String>())
        .collect::<Vec<_>>()
        .join(" "))
}

pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    let (gcd, mmi, _) = egcd(a, ALPHA.len() as i32);
    if gcd != 1 {
        return Err(AffineCipherError::NotCoprime(a));
    }
    Ok(ciphertext
        .to_lowercase()
        .chars()
        .filter(|c| !c.is_ascii_punctuation() && !c.is_whitespace())
        .map(|c| {
            let key = ALPHA.find(c).unwrap_or_default() as i32 - b;
            match c.is_alphabetic() {
                true => Some(
                    ALPHA
                        .chars()
                        .nth(((mmi * key).rem_euclid(ALPHA.len() as i32)) as usize)
                        .unwrap_or_default(),
                ),
                false => Some(c),
            }
        })
        .map(|w| w.unwrap())
        .collect::<String>())
}

fn egcd(a: i32, b: i32) -> (i32, i32, i32) {
    if a == 0 {
        return (b, 0, 1);
    }
    let (gcd, x1, y1) = egcd(b.rem_euclid(a), a);
    let x = y1 - (b.div_euclid(a)) * x1;
    let y = x1;
    (gcd, x, y)
}

// println!("ENC: {:?}", encode("This is a test.", 6, 17));
// println!("DEC: {:?}", decode("tytgn fjr", 3, 7));
// println!("ENC: {:?}", encode("Testing,1 2 3, testing.", 3, 4));
// println!(
//     "DEC: {:?}",
//     decode("swxtj npvyk lruol iejdc blaxk swxmh qzglf", 17, 33)
// );
// println!("DEC: {:?}", decode("odpoz ub123 odpoz ub", 25, 7));
