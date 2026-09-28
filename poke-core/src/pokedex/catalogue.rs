// // TODO: Finish FinalForm building
// impl Catalogue {
//     pub fn new(kind: DexKind, source: &Vec<PokemonSpecies>) -> Self {
//         let pokemon_array: Vec<DexIdentity> = match kind {
//             DexKind::FinalForm => source
//                 .iter()
//                 .flat_map(|species| {
//                     species.varieties.iter().flat_map(|variety| {
//                         variety
//                             .forms
//                             .iter()
//                             .take(1)
//                             .filter(|form| !form.is_battle_only)
//                             .enumerate()
//                             .map(|(i, form)| {
//                                 let mut f = form.clone();
//                                 f.order = i as u32;
//                                 DexIdentity::new(f, None)
//                             })
//                     })
//                 })
//                 .collect(),

//             DexKind::Living => {
//                 let mut counter: u32 = 0;

//                 source
//                     .iter()
//                     .flat_map(|species| {
//                         species
//                             .varieties
//                             .iter()
//                             .filter(|variety| variety.is_default)
//                             .flat_map(|variety| {
//                                 variety
//                                     .forms
//                                     .iter()
//                                     .filter(|form| !form.is_battle_only && form.is_default)
//                             })
//                     })
//                     .map(|form| {
//                         let mut f = form.clone();
//                         f.order = counter;
//                         counter += 1;

//                         DexIdentity::new(f, None)
//                     })
//                     .collect()
//             }

//             DexKind::LivingFormLite => {
//                 let mut counter: u32 = 0;

//                 source
//                     .iter()
//                     .flat_map(|species| {
//                         species.varieties.iter().flat_map(|variety| {
//                             variety.forms.iter().filter(|form| !form.is_battle_only)
//                         })
//                     })
//                     .map(|form| {
//                         let mut f = form.clone();
//                         f.order = counter;
//                         counter += 1;

//                         DexIdentity::new(f, None)
//                     })
//                     .collect()
//             }

//             DexKind::LivingForm => {
//                 let mut counter: u32 = 0;

//                 source
//                     .iter()
//                     .flat_map(|species| {
//                         species.varieties.iter().flat_map(|variety| {
//                             variety
//                                 .forms
//                                 .iter()
//                                 .filter(|form| !form.is_battle_only)
//                                 .map(|form| (species.has_gender_differences, form))
//                         })
//                     })
//                     .flat_map(|(has_gender_differences, form)| {
//                         // If the pokemon has gender differences and is the default form
//                         if has_gender_differences && form.is_default {
//                             let mut f1 = form.clone();
//                             let mut f2 = form.clone();

//                             f1.order = counter;
//                             counter += 1;
//                             f2.order = counter;
//                             counter += 1;

//                             vec![
//                                 DexIdentity::new(f1, Some(Gender::Male)),
//                                 DexIdentity::new(f2, Some(Gender::Female)),
//                             ]
//                         } else {
//                             let mut f = form.clone();
//                             f.order = counter;
//                             counter += 1;

//                             vec![DexIdentity::new(f, None)]
//                         }
//                     })
//                     .collect()
//             }
//         };

//         Catalogue { pokemon_array }
//     }
// }

use crate::pokedex::{DexEntry, DexEntryId, DexRegistry, PokemonSpecies};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum DexKind {
    FinalForm,
    Living,
    LivingFormLite,
    LivingForm,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SlotEntries {
    Single(DexEntryId),
    GenderPair(DexEntryId, DexEntryId),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CatalogueSlot {
    pub entries: SlotEntries,
    pub box_num: u32,
    pub row: u32,
    pub col: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Catalogue {
    pub kind: DexKind,
    pub slots: Vec<CatalogueSlot>,
}

impl CatalogueSlot {
    pub fn new(id: DexEntryId) -> Self {
        CatalogueSlot {
            entries: SlotEntries::Single(id),
            box_num: 0,
            row: 0,
            col: 0,
        }
    }
}

impl Catalogue {
    pub fn new(kind: DexKind, source: &[PokemonSpecies], registry: &mut DexRegistry) -> Self {
        let slots: Vec<CatalogueSlot> = match kind {
            DexKind::FinalForm => Vec::with_capacity(0),
            DexKind::Living => Vec::with_capacity(0),
            DexKind::LivingFormLite => source
                .iter()
                .flat_map(|species| {
                    species.varieties.iter().flat_map(|variety| {
                        variety.forms.iter().filter(|form| !form.is_battle_only)
                    })
                })
                .map(|form| {
                    // If form exists in a DexEntry, just use that else add to registry
                    let entry = registry.get(DexEntryId(form.form_id));

                    match entry {
                        Some(dex_entry) => CatalogueSlot::new(dex_entry.id),
                        None => {
                            let new_dex_entry =
                                DexEntry::new(DexEntryId(form.form_id), form.clone(), None);

                            registry.add(new_dex_entry);

                            CatalogueSlot::new(DexEntryId(form.form_id))
                        }
                    }
                })
                .collect(),
            DexKind::LivingForm => Vec::with_capacity(0),
        };

        Catalogue { kind, slots }
    }

    // pub fn resolve(&self, registry: &DexRegistry) -> Vec<DexIdentity> {

    // }
}
