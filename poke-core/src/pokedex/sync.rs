use crate::error::SyncError;
use crate::pokedex::{Species, Variety, Form};
use rustemon::Follow;
use rustemon::client::RustemonClient;
use rustemon::pokemon::pokemon_species;

pub async fn fetch_species(id: u32, client: &RustemonClient) -> Result<Species, SyncError> {
    let raw = pokemon_species::get_by_id(id as i64, client).await?;

    let mut varieties = Vec::with_capacity(raw.varieties.len());
    for variety in &raw.varieties {
        let pokemon = variety.pokemon.follow(client).await?;

        let mut forms = Vec::with_capacity(pokemon.forms.len());
        for form_ref in &pokemon.forms {
            let form = form_ref.follow(client).await?;
            forms.push(Form {
                form_id: form.id as u32,
                name: form.name,
                form_name: form.form_name,
                order: form.order as u32,
                is_default: form.is_default,
                is_battle_only: form.is_battle_only,
                is_mega: form.is_mega,
            });
        }

        varieties.push(Variety {
            pokemon_id: pokemon.id as u32,
            name: pokemon.name,
            order: pokemon.order as u32,
            is_default: variety.is_default,
            forms,
        });
    }

    Ok(Species {
        id: raw.id as u32,
        name: raw.name,
        order: raw.order as u32,
        generation: raw.generation.name,
        has_gender_differences: raw.has_gender_differences,
        gender_rate: raw.gender_rate as i8,
        is_legendary: raw.is_legendary,
        is_mythical: raw.is_mythical,
        is_baby: raw.is_baby,
        varieties: varieties,
    })
}