use crate::pokedex::PokemonForm;

#[derive(Debug, Clone, serde::Serialize)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Debug, Clone, serde::Serialize)]
pub enum CaughtStatus {
    NotCaught,
    Caught,
    InBox
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct DexIdentity {
    pub base: PokemonForm,
    pub gender: Option<Gender>,
    pub obtained: Option<CaughtStatus>,
    pub box_num: u32,
    pub row: u32,
    pub col: u32
}