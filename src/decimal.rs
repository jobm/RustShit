/// Type implementing arbitrary-precision decimal arithmetic
use std::{
    fmt::Debug,
    ops::{Add, Mul, Sub},
};

#[derive(Debug, PartialEq, Eq)]
pub enum Op {
    Addi,
    Subt,
    Muli,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Decimal(String);

impl Decimal {
    pub fn try_from(input: &str) -> Option<Decimal> {
        if input
            .chars()
            .all(|c| c.is_numeric() || c.eq(&'.') || c.eq(&'-'))
        {
            Some(Decimal(input.to_string()))
        } else {
            None
        }
    }

    fn value(&self, rhs: Decimal, op: Op) -> Self {
        let mut count: usize = 0;
        let mut decimal: Vec<String> = Vec::new();
        let has_minus = self.0.contains('-') || rhs.0.contains('-');
        let both_minus = self.0.contains('-') && rhs.0.contains('-');
        let check = |s: &str| s.chars().all(|c| c.is_ascii_digit()) || s.contains("-");
        self.0.split(".").zip(rhs.0.split(".")).for_each(|(a, b)| {
            if check(a) && check(b) {
                let mut op_res;
                if count == 0 {
                    // numbers before decimal point
                    op_res = Decimal::operate(a, b, &op, has_minus, both_minus)
                        .parse::<f64>()
                        .unwrap();
                    decimal.push(op_res.to_string());
                } else {
                    // numbers after decimal point
                    let new_lhs;
                    let new_rhs;
                    (new_lhs, new_rhs, op_res) =
                        Decimal::carry(decimal.pop().unwrap().as_str(), a, b, has_minus, &op);
                    decimal.push(op_res.to_string());
                    op_res = Decimal::operate(
                        new_lhs.as_str(),
                        new_rhs.as_str(),
                        &op,
                        has_minus,
                        both_minus,
                    )
                    .parse::<f64>()
                    .ok()
                    .unwrap();
                    let mut sres = String::new();
                    if op_res.to_string().chars().nth(0) == Some('1') {
                        let mut minus: bool = false;
                        if decimal.get(0).unwrap().chars().nth(0).eq(&Some('-')) {
                            minus = true;
                        }
                        let mut x: String =
                            (decimal.pop().unwrap().parse::<i64>().unwrap().abs() + 1).to_string();
                        if minus {
                            x.insert(0, '-');
                        }
                        decimal.push(x.to_string());
                        sres.push_str(op_res.to_string()[1..].to_string().as_str());
                    } else {
                        let z_count = || {
                            new_lhs
                                .len()
                                .min(new_rhs.len())
                                .saturating_sub(op_res.to_string().len())
                        };
                        sres.extend(["0".repeat(z_count()), op_res.to_string()]);
                    }
                    sres = sres.trim_end_matches('0').to_string();
                    sres = if sres.is_empty() {
                        "0".to_string()
                    } else {
                        sres
                    };
                    decimal.push(sres);
                }
                count += 1;
            }
        });
        Decimal(decimal.join("."))
    }

    fn align(lhs: &str, rhs: &str) -> ((String, String), usize) {
        let mut l = lhs.to_string();
        let mut r = rhs.to_string();
        let min_op = *[lhs, rhs].iter().min_by_key(|s| s.len()).unwrap();
        let max_op = *[lhs, rhs].iter().max_by_key(|s| s.len()).unwrap();
        let zero_count: usize = max_op.len() - min_op.len();
        if zero_count.eq(&0) {
            return ((l, r), 1);
        } else {
            let zeroes = std::iter::repeat(0)
                .take(zero_count)
                .map(|c| c.to_string())
                .collect::<String>();
            if min_op.eq(lhs) {
                l.extend([zeroes]);
            } else {
                r.extend([zeroes]);
            }
        }
        ((l, r), zero_count)
    }

    fn carry(
        prev_res: &str,
        lhs: &str,
        rhs: &str,
        has_minus: bool,
        op: &Op,
    ) -> (String, String, f64) {
        let mut new_lhs = String::from(lhs);
        let mut new_rhs = String::from(rhs);
        let mut carry_res = prev_res.parse::<f64>().ok().unwrap();
        if new_lhs.len() != new_rhs.len() {
            ((new_lhs, new_rhs), _) = Decimal::align(lhs, rhs);
        }
        if new_lhs < new_rhs && (op.eq(&Op::Subt) || has_minus) {
            carry_res -= 1f64;
            if carry_res < 0f64 {
                carry_res = 0f64;
            }
            new_lhs.insert(0, '1');
        }
        (new_lhs.to_string(), new_rhs.to_string(), carry_res)
    }

    fn operate(lhs: &str, rhs: &str, op: &Op, has_minus: bool, both_minus: bool) -> String {
        let part = match op {
            Op::Addi => {
                if has_minus && !both_minus {
                    lhs.parse::<f64>().unwrap() - rhs.parse::<f64>().unwrap()
                } else {
                    lhs.parse::<f64>().unwrap() + rhs.parse::<f64>().unwrap()
                }
            }
            Op::Muli => lhs.parse::<f64>().unwrap() + rhs.parse::<f64>().unwrap(),
            Op::Subt => {
                // borrow if lhs > rhs
                lhs.parse::<f64>().unwrap() + rhs.parse::<f64>().unwrap()
            }
        };
        part.to_string()
    }
}

// can we use a macro to avoid repeating add, mul, sub impls?
impl Add for Decimal {
    type Output = Decimal;
    fn add(self, rhs: Self) -> Self::Output {
        Decimal(Decimal::value(&self, rhs, Op::Addi).0)
    }
}

impl Mul for Decimal {
    type Output = Decimal;
    fn mul(self, rhs: Self) -> Self::Output {
        Decimal(Decimal::value(&self, rhs, Op::Muli).0)
    }
}

impl Sub for Decimal {
    type Output = Decimal;
    fn sub(self, rhs: Self) -> Self::Output {
        Decimal(Decimal::value(&self, rhs, Op::Subt).0)
    }
}
