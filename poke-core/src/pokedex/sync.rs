use crate::error::SyncError;
use crate::pokedex::{PokemonForm, PokemonSpecies, PokemonVariety};
use rustemon::Follow;
use rustemon::client::RustemonClient;
use rustemon::pokemon::{pokemon_form, pokemon_species};

pub async fn fetch_species(id: u32, client: &RustemonClient) -> Result<PokemonSpecies, SyncError> {
    let raw = pokemon_species::get_by_id(id as i64, client).await?;

    let mut varieties = Vec::with_capacity(raw.varieties.len());
    for variety in &raw.varieties {
        let pokemon = variety.pokemon.follow(client).await?;

        let mut forms = Vec::with_capacity(pokemon.forms.len());
        for form_ref in &pokemon.forms {
            let form = form_ref.follow(client).await?;
            forms.push(PokemonForm {
                form_id: form.id as u32,
                name: form.name,
                order: form.order as u32,
                is_default: form.is_default,
                is_battle_only: form.is_battle_only,
            });
        }

        varieties.push(PokemonVariety {
            pokemon_id: pokemon.id as u32,
            name: pokemon.name,
            order: pokemon.order as u32,
            is_default: pokemon.is_default,
            forms,
        });
    }

    Ok(PokemonSpecies {
        species_id: raw.id as u32,
        name: raw.name,
        order: raw.order as u32,
        has_gender_differences: raw.has_gender_differences,
        varieties: varieties,
    })
}

pub async fn fetch_form(id: u32, client: &RustemonClient) -> Result<PokemonForm, SyncError> {
    let raw = pokemon_form::get_by_id(id as i64, client).await?;

    Ok(PokemonForm {
        form_id: raw.id as u32,
        name: raw.name,
        order: raw.order as u32,
        is_default: raw.is_default,
        is_battle_only: raw.is_battle_only,
    })
}

// Function to get all HOME boxable entities
pub async fn fetch_catalogue(client: &RustemonClient) -> Result<Vec<PokemonSpecies>, SyncError> {
    let raw = pokemon_species::get_all_entries(client).await?;

    let mut catalogue: Vec<PokemonSpecies> = Vec::with_capacity(raw.len());
    // for form_ref in raw {
    //     let form = form_ref.follow(client).await?;
    //     catalogue.push(DexIdentity {
    //         form_id: form.id as u32,
    //         gender: None
    //     });
    // }

    for species_ref in &raw {
        let spec = species_ref.follow(client).await?;
        let mut varieties = Vec::with_capacity(spec.varieties.len());

        for variety_ref in &spec.varieties {
            let pokemon = variety_ref.pokemon.follow(client).await?;

            let mut forms = Vec::with_capacity(pokemon.forms.len());
            for form_ref in &pokemon.forms {
                let form = form_ref.follow(client).await?;

                forms.push(PokemonForm {
                    form_id: form.id as u32,
                    name: form.name,
                    order: form.order as u32,
                    is_default: form.is_default,
                    is_battle_only: form.is_battle_only
                });
            }

            varieties.push(PokemonVariety {
                pokemon_id: pokemon.id as u32,
                name: pokemon.name,
                order: pokemon.order as u32,
                is_default: pokemon.is_default,
                forms: forms
            });
        }

        catalogue.push(PokemonSpecies {
            species_id: spec.id as u32,
            name: spec.name,
            order: spec.order as u32,
            has_gender_differences: spec.has_gender_differences,
            varieties: varieties
        })
    }

    Ok(catalogue)
}
