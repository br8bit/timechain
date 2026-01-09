use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::u256;

#[derive(Clone, Copy, serde::Serialize)]
pub struct Hash(u256);

impl Hash {
    pub fn new<T: Serialize>(data: &T) -> Self {
        let mut serialized = vec![];
        if let Err(e) = ciborium::into_writer(data, &mut serialized) {
            panic!("Serialization error: {}", e);
        }

        let mut hasher = Sha256::new();
        hasher.update(&serialized);
        let hash_result = hasher.finalize();

        Hash(u256::from_big_endian(&hash_result))
    }

    pub fn zero() -> Self {
        Hash(u256::zero())
    }

    pub fn matches_target(&self, target: u256) -> bool {
        self.0 <= target
    }
}
