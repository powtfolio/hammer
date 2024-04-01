use serde::{Deserialize, Serialize};

#[derive(clap::ValueEnum, Clone, Default, Debug, Serialize)]
pub enum TokenStandard {
    #[default]
    Fungible,
    SemiFungible,
}

#[derive(Deserialize, Serialize)]
pub struct FungibleMetadata {
    // Name of the asset
    pub name: String,
    // Symbol of the asset
    pub symbol: String,
    // Description of the asset
    pub description: String,
    // URI pointing to the asset's logo
    pub image: String,
}
