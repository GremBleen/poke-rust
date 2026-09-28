use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokemonSpecies {
    pub species_id: u32,
    pub name: String,
    pub order: u32,
    pub has_gender_differences: bool,
    pub varieties: Vec<PokemonVariety>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokemonVariety {
    pub pokemon_id: u32,
    pub name: String,
    pub order: u32,
    pub is_default: bool,
    pub forms: Vec<PokemonForm>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokemonForm {
    pub form_id: u32,
    pub name: String,
    pub order: u32,
    pub is_default: bool,
    pub is_battle_only: bool,
}
