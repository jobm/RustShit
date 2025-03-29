use std::error::Error;
use std::{
    fmt::Debug,
    ops::{Add, Mul, Sub},
    str::FromStr,
};

/// Type implementing arbitrary-precision decimal arithmetic
pub enum DecimalOp {
    Addi,
    Subt,
    Muli,
}

#[derive(Debug)]
pub struct Decimal<T>(T);

impl<T> Decimal<T>
where
    T: ToString + FromStr,
{
    fn oparate(&self, rhs: Decimal<T>, op: DecimalOp) -> Self {
        let calc: String = self
            .0
            .to_string()
            .chars()
            .zip(rhs.0.to_string().chars())
            .map(|(a, b)| {
                if a.is_ascii_digit() && b.is_ascii_digit() {
                    let res = match op {
                        DecimalOp::Addi => a
                            .to_digit(10)
                            .unwrap()
                            .saturating_add(b.to_digit(10).unwrap()),
                        DecimalOp::Muli => a
                            .to_digit(10)
                            .unwrap()
                            .saturating_add(b.to_digit(10).unwrap()),
                        DecimalOp::Subt => a
                            .to_digit(10)
                            .unwrap()
                            .saturating_add(b.to_digit(10).unwrap()),
                    };
                    res.to_string()
                } else {
                    ".".to_owned()
                }
            })
            .collect();
        let val: T = T::from_str(calc.as_str()).ok().unwrap();
        Decimal(val)
    }
}

impl<T> TryFrom<&'static str> for Decimal<T>
where
    T: Add<Output = T> + FromStr + PartialEq + PartialOrd + Mul + Sub + ToString,
    T::Err: Error + 'static,
{
    type Error = Box<dyn Error + 'static>;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value: T = T::from_str(value)?;
        Ok(Decimal(value))
    }
}

impl<T> Add for Decimal<T>
where
    T: FromStr + ToString,
{
    type Output = Decimal<T>;
    fn add(self, rhs: Self) -> Self::Output {
        let value: T = Decimal::oparate(&self, rhs, DecimalOp::Addi).0;
        Decimal(value)
    }
}

impl<T> Mul for Decimal<T>
where
    T: FromStr + ToString,
{
    type Output = Decimal<T>;
    fn mul(self, rhs: Self) -> Self::Output {
        Decimal(Decimal::oparate(&self, rhs, DecimalOp::Muli).0)
    }
}

impl<T> Sub for Decimal<T>
where
    T: FromStr + ToString,
{
    type Output = Decimal<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        let value: T = Decimal::oparate(&self, rhs, DecimalOp::Subt).0;
        Decimal(value)
    }
}

// fn decmal(dec: &str) -> Decimal<T> {}
