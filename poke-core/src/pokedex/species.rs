#[derive(Debug, serde::Serialize, Clone)]
pub struct Species {
    pub id: u32,
    pub name: String,
    pub order: u32,
    pub generation: String,
    pub has_gender_differences: bool,
    pub gender_rate: i8,
    pub is_legendary: bool,
    pub is_mythical: bool,
    pub is_baby: bool,
    pub varieties: Vec<Variety>,
}

#[derive(Debug, serde::Serialize, Clone)]
pub struct Variety {
    pub pokemon_id: u32,
    pub name: String,
    pub order: u32,
    pub is_default: bool,
    pub forms: Vec<Form>,
}

#[derive(Debug, serde::Serialize, Clone)]
pub struct Form {
    pub form_id: u32,
    pub name: String,
    pub form_name: String,
    pub order: u32,
    pub is_default: bool,
    pub is_battle_only: bool,
    pub is_mega: bool,
}
