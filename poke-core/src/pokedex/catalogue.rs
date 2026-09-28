use crate::pokedex::{
    DexIdentity, PokemonForm, PokemonSpecies, PokemonVariety, identity::CaughtStatus::NotCaught,
};

pub trait DexType {
    fn populate(&self, catalogue: &mut Vec<PokemonSpecies>) -> Vec<DexIdentity>;
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Catalogue {
    pub pokemon_array: Vec<DexIdentity>,
}

impl Catalogue {
    pub fn populate_as(&mut self, dex_type: &impl DexType, catalogue: &mut Vec<PokemonSpecies>) {
        self.pokemon_array = dex_type.populate(catalogue);
    }
}

pub struct FinalForm;
pub struct Living;
pub struct LivingForm;
pub struct LivingFormGender;

// impl DexType for FinalForm {
//     // Needs to populate using the first form per species, but only where can evolve is false
//     fn populate(&self, catalogue: &mut Vec<PokemonSpecies>) -> Vec<DexIdentity> {
//         return Vec::with_capacity(0);
//     }
// }

impl DexType for Living {
    // Needs to populate using the first form per species
    fn populate(&self, catalogue: &mut Vec<PokemonSpecies>) -> Vec<DexIdentity> {
        let identities: Vec<DexIdentity> = catalogue
            .iter()
            .flat_map(|species: &PokemonSpecies| {
                species
                    .varieties
                    .iter()
                    .flat_map(|variety: &PokemonVariety| {
                        variety
                            .forms
                            .iter()
                            .take(1)
                            .filter(|form: &&PokemonForm| !form.is_battle_only)
                            .map(|form: &PokemonForm| DexIdentity {
                                base: form.clone(),
                                gender: None,
                                obtained: Some(NotCaught),
                                box_num: 0,
                                row: 0,
                                col: 0,
                            })
                    })
            })
            .collect();

        identities
    }
}

impl DexType for LivingForm {
    // Needs to populate using all forms
    fn populate(&self, catalogue: &mut Vec<PokemonSpecies>) -> Vec<DexIdentity> {
        let identities: Vec<DexIdentity> = catalogue
            .iter()
            .flat_map(|species| {
                species.varieties.iter().flat_map(|variety| {
                    variety
                        .forms
                        .iter()
                        .filter(|form| !form.is_battle_only)
                        .map(|form| DexIdentity {
                            base: form.clone(),
                            gender: None,
                            obtained: Some(NotCaught),
                            box_num: 0,
                            row: 0,
                            col: 0,
                        })
                })
            })
            .collect();

        identities
    }
}

// impl DexType for LivingFormGender {
//     // Needs to populate using all forms and have duplicate entries for when has_gender_differences is true
//     fn populate(&self) -> Vec<DexIdentity> {
//         return Vec::with_capacity(0)
//     }
// }
