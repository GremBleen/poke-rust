use poke_core::pokedex::{Species, fetch_species};
use rustemon::client::{CACacheManager, CacheMode, RustemonClientBuilder};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client= RustemonClientBuilder::<CACacheManager>::default()
        .with_mode(CacheMode::Default)
        .try_build()?;

    let species: Species = fetch_species(670, &client).await?;

    println!("{}", serde_json::to_string_pretty(&species)?);

    Ok(())
}