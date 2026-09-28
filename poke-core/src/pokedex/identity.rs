use crate::pokedex::{PokemonForm, identity::CaughtStatus::NotCaught};

#[derive(Debug, Clone, serde::Serialize)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Debug, Clone, serde::Serialize)]
pub enum CaughtStatus {
    NotCaught,
    Caught,
    InBox,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct DexIdentity {
    pub base: PokemonForm,
    pub gender: Option<Gender>,
    pub status: Option<CaughtStatus>,
    pub box_num: u32,
    pub row: u32,
    pub col: u32,
}

impl DexIdentity {
    pub fn new(form: PokemonForm, gender: Option<Gender>) -> Self {
        let box_num = (form.order / 30) + 1;

        let slot = form.order % 30;
        let row = slot / 6;
        let col = slot % 6;
        DexIdentity {
            base: form,
            gender,
            status: Some(NotCaught),
            box_num,
            row,
            col,
        }
    }
}
