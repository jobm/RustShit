#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span > string_digits.len() {
        return Err(Error::SpanTooLong);
    }
    let res = string_digits
        .chars()
        .collect::<Vec<_>>()
        .windows(span)
        .map(|w| {
            w.iter()
                .map(|&i| i.to_digit(10).ok_or(Error::InvalidDigit(i)))
                .collect::<Result<Vec<u32>, Error>>()
                .map(|vals| vals.iter().map(|&d| d as u64).product::<u64>())
        })
        .collect::<Result<Vec<u64>, _>>();
    match res {
        Ok(prod) => Ok(*prod.iter().max().unwrap()),
        Err(e) => Err(e),
    }
}
