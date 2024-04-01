use serde::{Deserialize, Serialize};

use crate::types;

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
