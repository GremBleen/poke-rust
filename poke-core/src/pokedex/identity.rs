use crate::pokedex::PokemonForm;

#[derive(Debug, Clone, serde::Serialize)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct DexIdentity {
    base: PokemonForm,
    pub gender: Option<Gender>
}