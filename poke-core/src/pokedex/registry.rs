use crate::pokedex::{DexEntry, DexEntryId, Gender, PokemonSpecies, identity::CaughtStatus};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DexRegistry {
    entries: Vec<DexEntry>,
}

// TODO: Might not need this
impl DexRegistry {
    pub fn default() -> Self {
        DexRegistry {
            entries: Vec::new(),
        }
    }

    pub fn build(source: &[PokemonSpecies]) -> Self {
        let entries = {
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
                            DexEntry::new(DexEntryId(f1.order), f1, Some(Gender::Male)),
                            DexEntry::new(DexEntryId(f2.order), f2, Some(Gender::Female)),
                        ]
                    } else {
                        let mut f = form.clone();
                        f.order = counter;
                        counter += 1;

                        vec![DexEntry::new(DexEntryId(f.order), f, None)]
                    }
                })
                .collect()
        };

        DexRegistry { entries }
    }

    // TODO
    pub fn get(&self, id: DexEntryId) -> Option<&DexEntry> {
        self.entries.iter().find(|dex_entry| dex_entry.id == id)
    }

    // TODO
    pub fn set_status(&mut self, id: DexEntryId, status: CaughtStatus) {}

    pub fn add(&mut self, entry: DexEntry) {
        let pos = self
            .entries
            .binary_search_by_key(&entry.id, |x| x.id)
            .unwrap_or_else(|e| e);
        self.entries.insert(pos, entry);
    }
}
