pub mod crypto;
pub mod sha256;
pub mod types;
pub mod utils;

use uint::construct_uint;

construct_uint! {
    /// 256-bit unsigned integer.
    pub struct u256(4);
}
