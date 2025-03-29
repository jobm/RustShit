use std::fmt::{Display, Formatter, Result};

pub struct Roman(String);

impl Roman {
    fn from_decimal(num: u32) -> String {
        let dec_roman = [
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
            (9, "IX"),
            (5, "V"),
            (1, "I"),
            (4, "IV"),
        ];
        let mut rem = num;
        let mut romans: Vec<&str> = Vec::new();
        for (dec, rom) in dec_roman {
            let curr_rem = rem;
            if curr_rem >= dec {
                rem = curr_rem.rem_euclid(dec);
                if dec.eq(&rem) {
                    romans.push(rom);
                    break;
                }
                romans.extend(std::iter::repeat(rom).take((curr_rem / dec) as usize));
            }
        }
        romans.join("")
    }
}

impl Display for Roman {
    fn fmt(&self, _f: &mut Formatter<'_>) -> Result {
        write!(_f, "{}", self.0)
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        Self(Roman::from_decimal(num))
    }
}
