pub struct Species {
    pub id: u32,
    pub name: String,
}

pub struct Form {
    pub slug: String,
    pub species_id: u32,
}

pub fn sample_species() -> Vec<Species> {
    vec![
        Species {
            id: 1,
            name: "Bulbasaur".into(),
        },
        Species {
            id: 2,
            name: "Squirtle".into(),
        },
        Species {
            id: 3,
            name: "Charmander".into()
        }
    ]
}
