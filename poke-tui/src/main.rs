use poke_core::pokedex::{Species, fetch_species, produce_order};
use rustemon::client::{CACacheManager, CacheMode, RustemonClientBuilder};
use poke_core::storage::repo::{save_json};
use std::{path::Path};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client= RustemonClientBuilder::<CACacheManager>::default()
        .with_mode(CacheMode::Default)
        .try_build()?;

    let species1: Species = fetch_species(670, &client).await?;
    let species2: Species = fetch_species(6, &client).await?;

    let path_string = format!("./species/{}.json", species1.name);
    let path: &Path = Path::new(&path_string);

    match save_json(path, &species1) {
        Ok(_) => {},
        Err(_) => {},
    };

    let path_string = format!("./species/{}.json", species2.name);
    let path: &Path = Path::new(&path_string);

    match save_json(path, &species2) {
        Ok(_) => {},
        Err(_) => {},
    };

    let path_string = format!("./species/order.txt");
    let path: &Path = Path::new(&path_string);

    let species_vec = Vec::from([species1, species2]);

    let vec_string: Vec<String> = produce_order(&species_vec);
    match save_json(path, &vec_string) {
        Ok(_) => {},
        Err(_) => {},
    };

    Ok(())
}