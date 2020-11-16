use crate::base::{Recipe, Spell};
use crate::vec::Vec4;

#[derive(Clone)]
pub struct Player {
    pub inventory: Vec4<u32>,
    pub score: i32,
    pub ready_spells: Vec<Spell>,
    pub used_spells: Vec<Spell>,
}

impl Player {
    pub fn new(inventory: Vec4<u32>, score: i32, spells: Vec<Spell>) -> Self {
        let mut ready_spells = vec![];
        let mut used_spells = vec![];
        for spell in spells.into_iter() {
            if spell.castable {
                ready_spells.push(spell);
            } else {
                used_spells.push(spell);
            }
        }
        Self {
            inventory,
            score,
            ready_spells,
            used_spells,
        }
    }

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
}
