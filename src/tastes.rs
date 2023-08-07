use std::hash::Hash;

use crate::prelude::*;

#[async_trait]
pub trait TasteStore: Send + Sync {

    fn hash_taste(taste: impl Hash) -> Vec<u8>;
    fn matching_rate(&self) -> f32;

    
}