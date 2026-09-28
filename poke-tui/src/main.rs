use poke_core::{pokedex::{Catalogue, DexKind, PokemonSpecies, fetch_catalogue}, storage::repo::load_json};
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

    let path_string = format!("./species/catalogue.jsonl");
    let path: &Path = Path::new(&path_string);

    let catalogue: Vec<PokemonSpecies> = match load_json(path) {
        Ok(source) => {
            println!("Waw");
            source},
        Err(_) => {fetch_catalogue(&client).await?}
    };

    match save_json(path, &catalogue) {
        Ok(_) => {},
        Err(_) => {},
    };

    // Construct Catalogues:
    let living_dex: Catalogue = Catalogue::new(DexKind::Living, &catalogue);

    let path_string = format!("./species/living_dex.jsonl");
    let path: &Path = Path::new(&path_string);

    match save_json(path, &living_dex) {
        Ok(_) => {},
        Err(_) => {},
    };


    let living_form_dex: Catalogue = Catalogue::new(DexKind::LivingForm, &catalogue);

    let path_string = format!("./species/living_form_dex.jsonl");
    let path: &Path = Path::new(&path_string);

    match save_json(path, &living_form_dex) {
        Ok(_) => {},
        Err(_) => {},
    };

    let living_form_gender_dex: Catalogue = Catalogue::new(DexKind::LivingFormGender, &catalogue);

    let path_string = format!("./species/living_form_gender_dex.jsonl");
    let path: &Path = Path::new(&path_string);

    match save_json(path, &living_form_gender_dex) {
        Ok(_) => {},
        Err(_) => {},
    };

    Ok(())
}