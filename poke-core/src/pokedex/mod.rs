mod species;
pub use species::{PokemonSpecies, PokemonVariety, PokemonForm};

mod identity;
pub use identity::{DexIdentity, Gender};

mod sync;
pub use sync::{fetch_species, fetch_form, fetch_catalogue};

mod catalogue;
pub use catalogue::{Catalogue, DexKind};