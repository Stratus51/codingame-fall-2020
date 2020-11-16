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

#[derive(Clone)]
pub struct Player {
    pub inventory: Vec4<u32>,
    pub score: i32,
    pub ready_spells: Vec<Spell>,
    pub used_spells: Vec<Spell>,
}

pub enum Action {
    Brew(i32),
    Cast(i32),
    Rest,
}

pub struct RecipeCost {
    pub turns: usize,
    pub waste: Vec4<u32>,
}

impl Player {
    pub fn can_brew(&self, recipe: &Recipe) -> bool {
        self.inventory[0] >= recipe.ingredients[0]
            && self.inventory[1] >= recipe.ingredients[1]
            && self.inventory[2] >= recipe.ingredients[2]
            && self.inventory[3] >= recipe.ingredients[3]
    }

    pub fn can_cast(&self, spell: &Spell) -> bool {
        self.inventory[0] as i32 + spell.delta[0] >= 0
            && self.inventory[1] as i32 + spell.delta[1] >= 0
            && self.inventory[2] as i32 + spell.delta[2] >= 0
            && self.inventory[3] as i32 + spell.delta[3] >= 0
    }

    pub fn required_ingredients(&self, cost: &Vec4<i32>) -> Vec4<i32> {
        [
            cost[0] - self.inventory[0] as i32,
            cost[1] - self.inventory[1] as i32,
            cost[2] - self.inventory[2] as i32,
            cost[3] - self.inventory[3] as i32,
        ]
        .into()
    }

    pub fn recipe_missing_ingredients(&self, recipe: &Recipe) -> Option<[u32; 4]> {
        let mut missing = [0u32; 4];
        let mut none_missing = true;
        for i in 0..4 {
            if recipe.ingredients[i] > self.inventory[i] {
                missing[i] = recipe.ingredients[i] - self.inventory[i];
                none_missing = false;
            }
        }
        if none_missing {
            None
        } else {
            Some(missing)
        }
    }

    // TODO Multi turn planning
    // pub fn evaluate_recipe_cost(&self, recipe: &Recipe) -> Option<RecipeCost> {
    //     let mut player = self.clone();
    //     let mut turns = 0;

    //     loop {
    //         let missing = match player.recipe_missing_ingredients(recipe) {
    //             Some(m) => m,
    //             None => {
    //                 let waste = [
    //                     player.inventory[0] - recipe.ingredients[0],
    //                     player.inventory[1] - recipe.ingredients[1],
    //                     player.inventory[2] - recipe.ingredients[2],
    //                     player.inventory[3] - recipe.ingredients[3],
    //                 ];
    //                 return Some(RecipeCost { turns, waste });
    //             }
    //         };
    //     }
    // }
}
