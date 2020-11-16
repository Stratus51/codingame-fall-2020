use crate::vec::Vec4;

#[derive(Clone)]
pub struct Recipe {
    // the unique ID of this spell or recipe
    pub id: i32,
    // ingredients change
    pub ingredients: Vec4<u32>,
    // The rpice in rupees if this is a potion
    pub price: u32,
}

#[derive(Clone)]
pub struct Spell {
    // the unique ID of this spell or recipe
    pub id: i32,
    // ingredients change
    pub delta: Vec4<i32>,
    // in the first two leagues: always 0; later: the index in the tome if this is a tome spell,
    // equal to the read-ahead tax; For brews, this is the value of the current urgency bonus
    pub tome_index: i32,
    // in the first two leagues: always 0; later: the amount of taxed tier-0 ingredients you gain
    // from learning this spell; For brews, this is how many times you can still gain an urgency bonus
    pub tax_count: i32,
    // in the first league: always 0; later: 1 if this is a castable player spell
    pub castable: bool,
    // for the first two leagues: always 0; later: 1 if this is a repeatable player spell
    pub repeatable: i32,
}

pub struct Usefulness {
    pub advancement: u32,
    pub regression: u32,
    pub cleaning: u32,
}

impl Spell {
    pub fn get_usefulness(&self, required: &Vec4<i32>) -> Usefulness {
        let mut advancement = 0;
        let mut regression = 0;
        let mut cleaning = 0;
        for i in 0..4 {
            if self.delta[i] * required[i] < 0 {
                regression += (self.delta[i] - required[i]).abs() as u32;
            } else {
                let delta = self.delta[i].abs();
                let required = required[i].abs();
                let progress = if delta > required { required } else { delta };
                if self.delta[i] >= 0 {
                    advancement += progress as u32;
                } else {
                    cleaning += progress as u32;
                }
            }
        }
        Usefulness {
            advancement,
            regression,
            cleaning,
        }
    }
}

pub struct Learn;

pub enum Action {
    Brew(i32),
    Cast(i32),
    Rest,
    Wait,
}

impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Brew(id) => write!(f, "BREW {}", id),
            Self::Cast(id) => write!(f, "CAST {}", id),
            Self::Rest => write!(f, "REST"),
            Self::Wait => write!(f, "WAIT"),
        }
    }
}
