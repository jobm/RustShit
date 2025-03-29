use std::u64;

pub fn encode(n: u64) -> String {
    if n == 0 {
        return String::from("zero");
    }
    let words = vec![
        "quintillions",
        "quadrillions",
        "trillion",
        "billion",
        "million",
        "thousand",
        "hundred",
        "ninety",
        "eighty",
        "seventy",
        "sixty",
        "fifty",
        "forty",
        "thirty",
        "twenty",
        "nineteen",
        "eighteen",
        "seventeen",
        "sixteen",
        "fifteen",
        "fourteen",
        "thirteen",
        "twelve",
        "eleven",
        "ten",
        "nine",
        "eight",
        "seven",
        "six",
        "five",
        "four",
        "three",
        "two",
        "one",
    ];

    let numerals = vec![
        1_000_000_000_000_000_000,
        1_000_000_000_000_000,
        1_000_000_000_000,
        1_000_000_000,
        1_000_000,
        1000,
        100,
        90,
        80,
        70,
        60,
        50,
        40,
        30,
        20,
        19,
        18,
        17,
        16,
        15,
        14,
        13,
        12,
        11,
        10,
        9,
        8,
        7,
        6,
        5,
        4,
        3,
        2,
        1,
    ];
    to_words(n, &numerals, &words)
}

fn to_words(n: u64, numerals: &Vec<u64>, words: &Vec<&str>) -> String {
    let mut res = String::new();
    for i in 0..numerals.len() {
        let val = numerals[i];
        let word = words[i];

        if n >= val {
            if n >= 100 {
                res.push_str(format!("{}{}", to_words(n / val, &numerals, words), " ").as_str());
            }
            res.push_str(word);
            if n % val > 0 {
                res.push_str(format!("{}{}", " ", to_words(n % val, &numerals, words)).as_str());
            }
            return res;
        }
    }
    res
}
