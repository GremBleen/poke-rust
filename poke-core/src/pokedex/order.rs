use crate::pokedex::{Species, Variety, Form};

pub fn produce_order(species: &Vec<Species>) -> Vec<String> {
    let mut forms: Vec<&Form> = species.iter().flat_map(|s: &Species| s.varieties.iter().flat_map(|v: &Variety| v.forms.iter().filter(|a| !a.is_battle_only))).collect();

    forms.sort_by_key(|f| f.order);

    forms.into_iter().map(|f| f.name.clone()).collect()
}