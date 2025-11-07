#[derive(Debug, PartialEq, Eq, Clone)]
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

impl From<Allergen> for u8 {
    fn from(allergen: Allergen) -> u8 {
        match allergen {
            Allergen::Eggs => 0,
            Allergen::Peanuts => 1,
            Allergen::Shellfish => 2,
            Allergen::Strawberries => 3,
            Allergen::Tomatoes => 4,
            Allergen::Chocolate => 5,
            Allergen::Pollen => 6,
            Allergen::Cats => 7,
        }
    }
}

impl TryFrom<u8> for Allergen {
    type Error = &'static str;
    
    fn try_from(index: u8) -> Result<Self, Self::Error> {
        match index {
            0 => Ok(Allergen::Eggs),
            1 => Ok(Allergen::Peanuts),
            2 => Ok(Allergen::Shellfish),
            3 => Ok(Allergen::Strawberries),
            4 => Ok(Allergen::Tomatoes),
            5 => Ok(Allergen::Chocolate),
            6 => Ok(Allergen::Pollen),
            7 => Ok(Allergen::Cats),
            _ => Err("No allergen exists for this index")
        }
    }
}

pub struct Allergies(u8);

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies(score as u8)
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let index: u8 = u8::from(allergen.clone());
        self.0 >> index & 1 == 1
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut allergies = Vec::new();
        for index in 0..8 {
            if self.0 >> index & 1 == 1 {
                allergies.push(Allergen::try_from(index).unwrap())
            }
        }
        allergies
    }
}
