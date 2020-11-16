mod vec;
//
mod base;
mod input;
mod player;

const ME: usize = input::ME;
const MEAN_PRICE: f32 = 10.0;

fn play() -> base::Action {
    let input = input::Input::parse();

    // If we can make a recipe, make it
    let best_recipe = input
        .recipes
        .iter()
        .filter(|r| input.players[ME].can_brew(&r))
        .max_by(|a, b| a.price.cmp(&b.price));

    if let Some(act) = best_recipe {
        return base::Action::Brew(act.id);
    }

    // Else find the most cost efficient spell
    let me = &input.players[ME];

    // Find unused spells
    let spells: Vec<_> = me
        .ready_spells
        .iter()
        .filter(|spell| me.can_cast(spell))
        .collect();

    // If every spell is used, rest
    if spells.is_empty() {
        return base::Action::Rest;
    }

    let (_, best_spell, best_score) = input
        .recipes
        .iter()
        .map(|r| {
            let (spell, score) = spells
                .iter()
                .map(|spell| {
                    let required = me.required_ingredients(&r.ingredients.clone().into());
                    let usefulness = spell.get_usefulness(&required);
                    let nb_required = required.positive().norm1();
                    let score = (usefulness.advancement as f32 / nb_required as f32)
                        * r.price as f32
                        / MEAN_PRICE
                        - usefulness.regression as f32
                        + usefulness.cleaning as f32;
                    (spell, score)
                })
                .max_by(|(_, a_score), (_, b_score)| a_score.partial_cmp(&b_score).unwrap())
                .unwrap();
            (r, spell, score)
        })
        .max_by(|a, b| a.2.partial_cmp(&b.2).unwrap())
        .unwrap();

    if best_score < 0.0 && me.inventory.norm1() >= 8 {
        base::Action::Rest
    } else {
        base::Action::Cast(best_spell.id)
    }
}

// TODO Evaluate a recipe real time cost using the spell combinations
// TODO Target a specific recipe

// TODO Find the same information for the opponent
// Check if accumulating ingredients before his next recipe is more worth it

fn main() {
    // game loop
    loop {
        println!("{}", play());
    }
}
