use crate::pokedex::{DexIdentity, Gender, PokemonSpecies};

#[derive(Debug, Clone, serde::Serialize)]
pub struct Catalogue {
    pub pokemon_array: Vec<DexIdentity>,
}

pub enum DexKind {
    FinalForm,
    Living,
    LivingForm,
    LivingFormGender,
}

// TODO: Fix box, col and row number calculations | Finish FinalForm building
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
                                DexIdentity::new(f, None, 0, 0)
                            })
                    })
                })
                .collect(),

            DexKind::Living => source
                .iter()
                .flat_map(|species| {
                    species.varieties.iter().take(1).flat_map(|variety| {
                        variety
                            .forms
                            .iter()
                            .take(1)
                            .filter(|form| !form.is_battle_only)
                    })
                })
                .enumerate()
                .map(|(i, form)| {
                    let mut f = form.clone();
                    f.order = i as u32;
                    DexIdentity::new(f, None, 0, 0)
                })
                .collect(),

            DexKind::LivingForm => source
                .iter()
                .flat_map(|species| {
                    species.varieties.iter().flat_map(|variety| {
                        variety.forms.iter().filter(|form| !form.is_battle_only)
                    })
                })
                .enumerate()
                .map(|(i, form)| {
                    let mut f = form.clone();
                    f.order = i as u32;
                    DexIdentity::new(f, None, 0, 0)
                })
                .collect(),

            DexKind::LivingFormGender => source
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
                .enumerate()
                .flat_map(|(mut i, (has_gender_differences, form))| {
                    if has_gender_differences {
                        let mut f1 = form.clone();
                        let mut f2 = form.clone();

                        f1.order = i as u32;
                        i = i + 1;
                        f2.order = i as u32;

                        vec![
                            DexIdentity::new(f1, Some(Gender::Male), 0, 0),
                            DexIdentity::new(f2, Some(Gender::Female), 0, 0),
                        ]
                    } else {
                        let mut f = form.clone();

                        f.order = i as u32;
                        vec![DexIdentity::new(f, None, 0, 0)]
                    }
                })
                .collect(),
        };

        Catalogue { pokemon_array }
    }
}
