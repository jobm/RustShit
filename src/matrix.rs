use std::io::ErrorKind::InvalidInput;

pub struct Matrix {
    rows: Vec<Vec<u32>>,
    columns: Vec<Vec<u32>>,
}

impl Matrix {
    pub fn new(input: &str) -> Self {
        let rows = input
            .lines()
            .map(|row| {
                row.split(char::is_whitespace)
                    .map(|c| c.parse::<u32>().unwrap())
                    .collect()
            })
            .collect::<Vec<Vec<u32>>>();
        let columns = rows.clone();
        Matrix { rows, columns }
    }

    pub fn row(&self, row_no: usize) -> Option<Vec<u32>> {
        self.rows.get(row_no - 1).take().cloned()
    }

    pub fn column(&self, col_no: usize) -> Option<Vec<u32>> {
        let cols = self
            .columns
            .iter()
            .map(|row| row.get(col_no - 1).ok_or(InvalidInput))
            .collect::<Result<Vec<&u32>, _>>();
        match cols {
            Err(_) => None,
            Ok(cols) => Some(cols.iter().map(|&c| *c).collect()),
        }
    }
}
