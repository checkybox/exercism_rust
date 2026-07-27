pub struct Allergies {
    allergens: Vec<Allergen>
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
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

fn num_to_allergen(num: u32) -> Option<Allergen> {
    match num {
        1 => Some(Allergen::Eggs),
        2 => Some(Allergen::Peanuts),
        4 => Some(Allergen::Shellfish),
        8 => Some(Allergen::Strawberries),
        16 => Some(Allergen::Tomatoes),
        32 => Some(Allergen::Chocolate),
        64 => Some(Allergen::Pollen),
        128 => Some(Allergen::Cats),
        _ => None
    }
}

impl Allergies {
    const VALID_VALUES: [u32; 8] = [1, 2, 4, 8, 16, 32, 64, 128];

    pub fn new(score: u32) -> Self {
        let mut allergens = Vec::new();

        for &value in &Self::VALID_VALUES {
            if score & value != 0 {
                allergens.push(num_to_allergen(value).unwrap());
            }
        }

        Self { allergens }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergens.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.allergens.clone()
    }
}
