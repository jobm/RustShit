#[derive(Debug)]
pub struct Bitset {
    pub bits: Vec<u8>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Bitset {
    pub fn new(size: i32) -> Self {
        Bitset {
            bits: vec![0; size as usize],
        }
    }

    pub fn fix(&mut self, idx: i32) {
        self.shift(idx as usize, 1)
    }

    pub fn unfix(&mut self, idx: i32) {
        self.shift(idx as usize, 0)
    }

    pub fn flip(&mut self) {
        self.bits.iter_mut().for_each(|b| {
            *b = if *b == 0 { 1 } else { 0 };
        });
    }

    pub fn all(&self) -> bool {
        self.bits.len() == self.count() as usize
    }

    pub fn one(&self) -> bool {
        self.count() >= 1
    }

    pub fn count(&self) -> i32 {
        self.bits.iter().copied().map(i32::from).sum()
    }

    pub fn to_string(&self) -> String {
        self.bits.iter().map(|b| format!("{}", b)).collect()
    }

    fn shift(&mut self, idx: usize, bit: u8) {
        if let Some(b) = self.bits.iter_mut().into_slice().get_mut(idx) {
            *b = bit;
        }
    }
}
