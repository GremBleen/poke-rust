use crate::pokedex::PokemonForm;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CaughtStatus {
    NotCaught,
    Caught,
    InBox,
}

// TODO: Define proper ordinance logic for two caught statuses
impl CaughtStatus {
    pub fn combine(a: &CaughtStatus, b: &CaughtStatus) -> CaughtStatus {
        CaughtStatus::NotCaught
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct DexEntryId(pub u32);

#[derive(Debug, Serialize, Deserialize)]
pub struct DexEntry {
    pub id: DexEntryId,
    pub base: PokemonForm,
    pub gender: Option<Gender>, // Some(_) only for the split leaves of a gender-diff species
    pub status: CaughtStatus,
}

impl DexEntry {
    pub fn new(id: DexEntryId, form: PokemonForm, gender: Option<Gender>) -> Self {
        DexEntry {
            id,
            base: form,
            gender,
            status: CaughtStatus::NotCaught,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct DexIdentity {
    pub base: PokemonForm,
    pub gender: Option<Gender>,
    pub status: CaughtStatus,
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
            status: CaughtStatus::NotCaught,
            box_num,
            row,
            col,
        }
    }
}
