mod species;
pub use species::{PokemonSpecies, PokemonVariety, PokemonForm};

mod identity;
pub use identity::{DexIdentity, Gender, DexEntry, DexEntryId};

mod sync;
pub use sync::{fetch_species, fetch_form, fetch_catalogue};

mod catalogue;
pub use catalogue::{Catalogue, DexKind};

mod registry;
pub use registry::{DexRegistry};