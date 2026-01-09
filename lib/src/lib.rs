pub mod crypto;
pub mod sha256;
pub mod types;
pub mod utils;

use serde::{Deserialize, Serialize};
use uint::construct_uint;

construct_uint! {
    /// 256-bit unsigned integer.
    #[derive(Serialize, Deserialize)]
    pub struct u256(4);
}
