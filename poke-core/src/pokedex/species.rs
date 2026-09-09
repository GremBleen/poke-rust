#[derive(Debug, serde::Serialize, Clone)]
pub struct PokemonSpecies {
    pub species_id: u32,
    pub name: String,
    pub order: u32,
    pub varieties: Vec<PokemonVariety>,
}

#[derive(Debug, serde::Serialize, Clone)]
pub struct PokemonVariety {
    pub pokemon_id: u32,
    pub name: String,
    pub order: u32,
    pub forms: Vec<PokemonForm>,
}

#[derive(Debug, serde::Serialize, Clone)]
pub struct PokemonForm {
    pub form_id: u32,
    pub name: String,
    pub order: u32,
    pub is_battle_only: bool,
}
