#[derive(Debug, Clone, Copy)]
pub enum Gender {
    Male,
    Female
}

#[derive(Debug, Clone, Copy)]
pub struct DexIdentity {
    pub species_id: u32,
    pub form_id: Option<u32>,
    pub gender: Option<Gender>
}