use crate::pokedex::{DexIdentity, Gender, PokemonSpecies};

#[derive(Debug, Clone, serde::Serialize)]
pub struct Catalogue {
    pub pokemon_array: Vec<DexIdentity>,
}

pub enum DexKind {
    FinalForm,      // The final evolutions of every evolutionary line
    Living,         // Every unique pokemon species
    LivingFormLite, // Includes Living dex but adds all alternate forms of each pokemon
    LivingForm,     // Includes the LivingFormLite dex, but adds gender differences
}

// TODO: Finish FinalForm building
impl Catalogue {
    pub fn new(kind: DexKind, source: &Vec<PokemonSpecies>) -> Self {
        let pokemon_array: Vec<DexIdentity> = match kind {
            DexKind::FinalForm => source
                .iter()
                .flat_map(|species| {
                    species.varieties.iter().flat_map(|variety| {
                        variety
                            .forms
                            .iter()
                            .take(1)
                            .filter(|form| !form.is_battle_only)
                            .enumerate()
                            .map(|(i, form)| {
                                let mut f = form.clone();
                                f.order = i as u32;
                                DexIdentity::new(f, None)
                            })
                    })
                })
                .collect(),

            DexKind::Living => {
                let mut counter: u32 = 0;

                source
                    .iter()
                    .flat_map(|species| {
                        species
                            .varieties
                            .iter()
                            .filter(|variety| variety.is_default)
                            .flat_map(|variety| {
                                variety
                                    .forms
                                    .iter()
                                    .filter(|form| !form.is_battle_only && form.is_default)
                            })
                    })
                    .map(|form| {
                        let mut f = form.clone();
                        f.order = counter;
                        counter += 1;

                        DexIdentity::new(f, None)
                    })
                    .collect()
            }

            DexKind::LivingFormLite => {
                let mut counter: u32 = 0;

                source
                    .iter()
                    .flat_map(|species| {
                        species.varieties.iter().flat_map(|variety| {
                            variety.forms.iter().filter(|form| !form.is_battle_only)
                        })
                    })
                    .map(|form| {
                        let mut f = form.clone();
                        f.order = counter;
                        counter += 1;

                        DexIdentity::new(f, None)
                    })
                    .collect()
            }

            DexKind::LivingForm => {
                let mut counter: u32 = 0;

                source
                    .iter()
                    .flat_map(|species| {
                        species.varieties.iter().flat_map(|variety| {
                            variety
                                .forms
                                .iter()
                                .filter(|form| !form.is_battle_only)
                                .map(|form| (species.has_gender_differences, form))
                        })
                    })
                    .flat_map(|(has_gender_differences, form)| {
                        // If the pokemon has gender differences and is the default form
                        if has_gender_differences && form.is_default {
                            let mut f1 = form.clone();
                            let mut f2 = form.clone();

                            f1.order = counter;
                            counter += 1;
                            f2.order = counter;
                            counter += 1;

                            vec![
                                DexIdentity::new(f1, Some(Gender::Male)),
                                DexIdentity::new(f2, Some(Gender::Female)),
                            ]
                        } else {
                            let mut f = form.clone();
                            f.order = counter;
                            counter += 1;

                            vec![DexIdentity::new(f, None)]
                        }
                    })
                    .collect()
            }
        };

        Catalogue { pokemon_array }
    }
}
