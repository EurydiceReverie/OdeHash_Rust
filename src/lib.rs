pub mod field256;
pub mod permutation256;
pub mod sponge256;

use sponge256::{Sponge256, ODE_HASH256};

/// Compute ODE-Hash v5 (256-bit field) on input bytes.
pub fn ode_hash_v5(input: &[u8]) -> [u8; 32] {
    let mut s = Sponge256::new(ODE_HASH256);
    s.update(input);
    s.finalize()
}

/// Compute ODE-Hash v5 and return hex string.
pub fn ode_hash_v5_hex(input: &[u8]) -> String {
    ode_hash_v5(input).iter().map(|b| format!("{:02x}", b)).collect()
}

/// Compute ODE-Hash v5 with domain separation.
pub fn ode_hash_v5_with_domain(input: &[u8], domain: u64) -> [u8; 32] {
    let mut s = Sponge256::new(domain);
    s.update(input);
    s.finalize()
}
