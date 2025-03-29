use std::collections::HashMap;

pub struct Roman;

impl Roman {
    pub fn parts_num<'a>(mut number: u32) -> Vec<&'a str> {
        // let mut parts = Vec::new();
        let bases: HashMap<u32, u32> = HashMap::from([(1, 1), (2, 10), (3, 100), (4, 1000)]); // find a better way
        let dec_roman: HashMap<u32, &str> = HashMap::from([
            (1, "I"),
            (2, "II"),
            (3, "III"),
            (4, "IV"),
            (5, "V"),
            (6, "VI"),
            (7, "VII"),
            (8, "VIII"),
            (9, "IX"),
            (10, "X"),
            (40, "XL"),
            (50, "L"),
            (90, "XC"),
            (100, "C"),
            (400, "CD"),
            (500, "D"),
            (900, "CM"),
            (1000, "M"),
        ]);
        let mut prev_base = 0;
        let mut key = number.to_string().len();
        let mut parts: Vec<u32> = Vec::new();
        let mut roman_n: Vec<&str> = Vec::new();
        let mut curr_roman: Vec<&str> = Vec::new();
        let mut final_count: u32 = 0;
        while key > 0 {
            let mut count_n = 0;
            let base = *bases.get(&(key as u32)).unwrap();
            while number >= base {
                count_n += base;
                if let Some(roman) = dec_roman.get(&count_n) {
                    curr_roman.push(*roman);
                }
                if prev_base > 0 && prev_base != base {
                    println!("X: {roman_n:?}, {:?}", (prev_base, base));
                    if curr_roman.pop().is_some() {
                        roman_n.push(curr_roman.pop().unwrap());
                    }
                    count_n = base;
                    prev_base = base;
                    curr_roman = Vec::new();
                }
                number -= base;
                parts.push(base);
            }
            key -= 1;
            prev_base = base;
            final_count = count_n;
        }
        if final_count > 0 {
            roman_n.push(dec_roman.get(&final_count).unwrap_or(&"I"));
        }
        println!("P: {:?}", parts);
        roman_n
    }
}
