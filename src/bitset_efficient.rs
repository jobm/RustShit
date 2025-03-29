#[derive(Debug)]
pub struct Bitset {
    size: i32,
    bits: Vec<bool>,
    flipped: bool,
    ones_count: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Bitset {
    pub fn new(size: i32) -> Self {
        Bitset {
            size,
            bits: vec![false; size as usize],
            flipped: false,
            ones_count: 0,
        }
    }

    pub fn fix(&mut self, idx: i32) {
        let idx = idx as usize;
        if self.flipped {
            if self.bits[idx] {
                self.bits[idx] = false;
                self.ones_count += 1;
            }
        } else {
            if !self.bits[idx] {
                self.bits[idx] = true;
                self.ones_count += 1;
            }
        }
    }

    pub fn unfix(&mut self, idx: i32) {
        let idx = idx as usize;
        if self.flipped {
            if !self.bits[idx] {
                self.bits[idx] = true;
                self.ones_count -= 1;
            }
        } else {
            if self.bits[idx] {
                self.bits[idx] = false;
                self.ones_count -= 1;
            }
        }
    }

    pub fn flip(&mut self) {
        self.flipped = !self.flipped;
        self.ones_count = self.size as usize - self.ones_count;
    }

    pub fn all(&self) -> bool {
        self.ones_count == self.size as usize
    }

    pub fn one(&self) -> bool {
        self.ones_count > 0
    }

    pub fn count(&self) -> usize {
        self.ones_count
    }

    pub fn to_string(&self) -> String {
        self.bits
            .iter()
            .map(|&bit| {
                if self.flipped {
                    if bit {
                        '0'
                    } else {
                        '1'
                    }
                } else {
                    if bit {
                        '1'
                    } else {
                        '0'
                    }
                }
            })
            .collect()
    }
}
