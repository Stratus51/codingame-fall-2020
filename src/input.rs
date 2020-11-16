use crate::base::{Learn, Player, Recipe, Spell};

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

pub enum InputAction {
    Brew(Recipe),
    Cast(Spell),
    OpponentCast(Spell),
    Learn(Learn),
}

impl InputAction {
    pub fn parse() -> Self {
        let mut input_line = String::new();
        std::io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(' ').collect::<Vec<_>>();
        let ty = inputs[1].trim();
        match ty {
            // in the first league: BREW; later: CAST, OPPONENT_CAST, LEARN, BREW
            "BREW" => Self::Brew(Recipe {
                id: parse_input!(inputs[0], i32),
                ingredients: [
                    -parse_input!(inputs[2], i32) as u32,
                    -parse_input!(inputs[3], i32) as u32,
                    -parse_input!(inputs[4], i32) as u32,
                    -parse_input!(inputs[5], i32) as u32,
                ]
                .into(),
                price: parse_input!(inputs[6], u32),
            }),
            "CAST" => Self::Cast(Spell {
                id: parse_input!(inputs[0], i32),
                delta: [
                    parse_input!(inputs[2], i32),
                    parse_input!(inputs[3], i32),
                    parse_input!(inputs[4], i32),
                    parse_input!(inputs[5], i32),
                ]
                .into(),
                tome_index: parse_input!(inputs[7], i32),
                tax_count: parse_input!(inputs[8], i32),
                castable: parse_input!(inputs[9], i32) == 1,
                repeatable: parse_input!(inputs[10], i32),
            }),
            "OPPONENT_CAST" => Self::OpponentCast(Spell {
                id: parse_input!(inputs[0], i32),
                delta: [
                    parse_input!(inputs[2], i32),
                    parse_input!(inputs[3], i32),
                    parse_input!(inputs[4], i32),
                    parse_input!(inputs[5], i32),
                ]
                .into(),
                tome_index: parse_input!(inputs[7], i32),
                tax_count: parse_input!(inputs[8], i32),
                castable: parse_input!(inputs[9], i32) == 1,
                repeatable: parse_input!(inputs[10], i32),
            }),
            "LEARN" => Self::Learn(Learn),
            x => panic!("Unknown action type: {}", x),
        }
    }
}

pub const ME: usize = 0;
pub const OPPONENT: usize = 1;

pub struct Input {
    pub recipes: Vec<Recipe>,
    pub players: [Player; 2],
}

impl Input {
    pub fn parse_player(mut spells: Vec<Spell>) -> Player {
        let mut input_line = String::new();
        std::io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(' ').collect::<Vec<_>>();
        let mut ready_spells = vec![];
        let mut used_spells = vec![];
        for spell in spells.into_iter() {
            if spell.castable {
                ready_spells.push(spell);
            } else {
                used_spells.push(spell);
            }
        }
        Player {
            inventory: [
                parse_input!(inputs[0], u32), // tier-0 ingredients in inventory
                parse_input!(inputs[1], u32),
                parse_input!(inputs[2], u32),
                parse_input!(inputs[3], u32),
            ]
            .into(),
            score: parse_input!(inputs[4], i32), // amount of rupees
            ready_spells,
            used_spells,
        }
    }

    pub fn parse() -> Self {
        let mut input_line = String::new();
        std::io::stdin().read_line(&mut input_line).unwrap();
        let action_count = parse_input!(input_line, i32); // the number of spells and recipes in play
        let actions: Vec<_> = (0..action_count).map(|_| InputAction::parse()).collect();
        let mut my_spells = vec![];
        let mut opp_spells = vec![];
        let mut recipes = vec![];
        for action in actions.into_iter() {
            match action {
                InputAction::Brew(recipe) => recipes.push(recipe),
                InputAction::Cast(spell) => my_spells.push(spell),
                InputAction::OpponentCast(spell) => opp_spells.push(spell),
                InputAction::Learn(_) => (),
            }
        }
        Self {
            recipes,
            players: [
                Self::parse_player(my_spells),
                Self::parse_player(opp_spells),
            ],
        }
    }
}
