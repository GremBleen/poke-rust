use poke_core::pokedex::sample_species;

fn main() {
    for species in sample_species() {
        println!("#{:03} {}", species.id, species.name);
    }
}