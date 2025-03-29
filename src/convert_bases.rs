#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    // let x = number.iter().map(|n|  ).collect::<Vec<_>>();
    println!("My X: {:?}", char::from_u32(101).unwrap_or('0'));
    Ok(Vec::new())
}
// input_base = 97 - 73;
//  let input_digits = &[3, 46, 60];
//  let output_base = 73;
// (4 × 10¹) + (2 × 10⁰)
// 34660 -> (3 * 24^4) + (4 * 73^3) + (6 * 73^2) + (6 * 73^1) + (7 * 73^0)
// (3 * 73²) + (46 * 73¹) + (60 * 73⁰) = 32749 - > (3 * 97^4) + (4 * 97^3) + (6 * 97^2) + (6 * 97^1) + (0 * 97^0)
// (46 * 97 ^ 0) + (46 * 73 ^ 0) = 92
// (60 * 97 ^ 0) + (60 * 73 ^ 0) = 120
//
//
//
//
//
//
// 0 * 3 ^ 2 + 6 * 3 ^ 1 + 0 * 3 ^ 0
//
// 2 * 16^2 + 1 * 16^1 + 0 * 16^0 528 = 5 * 3^2 + 2 * 3^1 + 8 * 3^0
