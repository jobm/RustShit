use std::collections::HashMap;

pub struct Allergies(Vec<Allergen>);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        let upper_bound = 256;
        let allergies_map: HashMap<u32, Allergen> = HashMap::from([
            (1, Allergen::Eggs),
            (2, Allergen::Peanuts),
            (4, Allergen::Shellfish),
            (8, Allergen::Strawberries),
            (16, Allergen::Tomatoes),
            (32, Allergen::Chocolate),
            (64, Allergen::Pollen),
            (128, Allergen::Cats),
        ]);
        let mut rem_score = score;
        let mut allergies: Vec<Allergen> = Vec::new();
        if score >= upper_bound {
            rem_score = score.rem_euclid(upper_bound);
        }
        if let Some(allergy) = allergies_map.get(&rem_score) {
            allergies.push(*allergy);
        } else {
            let mut i = 1;
            let v: &[u32] = &[128, 64, 32, 16, 8, 4, 2, 1];
            let mut temp: u32 = rem_score.saturating_sub(*v.first().unwrap());
            if temp > 0 {
                allergies.push(*allergies_map.get(v.first().unwrap()).unwrap())
            } else {
                temp = rem_score;
            }
            while i <= v.len() as u32 {
                let mut rem = temp;
                if rem == 0 {
                    break;
                }
                if rem.saturating_sub(*v.get(i as usize).unwrap_or(&0)) > 0 {
                    if allergies_map.contains_key(&rem) {
                        allergies.push(*allergies_map.get(&rem).unwrap());
                        rem -= rem;
                    } else {
                        let idx = v.get(i as usize).unwrap_or(&0);
                        allergies.push(*allergies_map.get(idx).unwrap());
                    }
                } else {
                    i += 1;
                    continue;
                }
                rem = rem.saturating_sub(*v.get(i as usize).unwrap_or(&0));
                temp = rem;
                i += 1;
            }
        }
        Allergies(allergies)
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.0.iter().any(|allergy| allergy.eq(allergen))
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.0.to_vec()
    }
}
