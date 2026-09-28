use poke_core::pokedex::{Catalogue, fetch_catalogue, Living, LivingForm};
use rustemon::client::{CACacheManager, CacheMode, RustemonClientBuilder};
use poke_core::storage::repo::{save_json};
use std::{path::Path};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client= RustemonClientBuilder::<CACacheManager>::default()
        .with_mode(CacheMode::Default)
        .try_build()?;

    // let species1: PokemonSpecies = fetch_species(670, &client).await?;
    // let species2: PokemonSpecies = fetch_species(6, &client).await?;

    // let path_string = format!("./species/{}.json", species1.name);
    // let path: &Path = Path::new(&path_string);

    // match save_json(path, &species1) {
    //     Ok(_) => {},
    //     Err(_) => {},
    // };

    // let path_string = format!("./species/{}.json", species2.name);
    // let path: &Path = Path::new(&path_string);

    // match save_json(path, &species2) {
    //     Ok(_) => {},
    //     Err(_) => {},
    // };

    // let path_string = format!("./species/order.txt");
    // let path: &Path = Path::new(&path_string);

    // let species_vec = Vec::from([species1, species2]);

    // let vec_string: Vec<String> = produce_order(&species_vec);
    // match save_json(path, &vec_string) {
    //     Ok(_) => {},
    //     Err(_) => {},
    // };

    let catalogue = fetch_catalogue(&client).await?;

    // Construct Catalogues:
    let mut living_dex: Catalogue = Catalogue { pokemon_array: Vec::new() };
    living_dex.populate_as(&Living, &mut catalogue.clone());

    let path_string = format!("./species/living_dex.jsonl");
    let path: &Path = Path::new(&path_string);

    match save_json(path, &living_dex) {
        Ok(_) => {},
        Err(_) => {},
    };

    // Construct Catalogues:
    let mut living_form_dex: Catalogue = Catalogue { pokemon_array: Vec::new() };
    living_form_dex.populate_as(&LivingForm, &mut catalogue.clone());

    let path_string = format!("./species/living_form_dex.jsonl");
    let path: &Path = Path::new(&path_string);

    match save_json(path, &living_form_dex) {
        Ok(_) => {},
        Err(_) => {},
    };

    Ok(())
}