use ode_hash_v4::field256::*;
use ode_hash_v4::permutation256::*;

fn main() {
    println!("=== Rust v5 Debug ===\n");

    // test field: 2 * 3 = 6
    let two = Fe256::from_u64(2);
    let three = Fe256::from_u64(3);
    let six = two.mul(three);
    println!("  2*3 = {:016x} {:016x} {:016x} {:016x}", six.0[3], six.0[2], six.0[1], six.0[0]);

    // test field: inv(2)
    let inv2 = two.inv();
    let check = two.mul(inv2);
    println!("  2*inv(2) = {:016x} {:016x} {:016x} {:016x}", check.0[3], check.0[2], check.0[1], check.0[0]);

    // IV words (same as sponge256.rs)
    let iv: [u64; 16] = [
        0x6a09e667f3bcc908, 0xbb67ae8584caa73b,
        0x3c6ef372fe94f82b, 0xa54ff53a5f1d36f1,
        0x510e527fade682d1, 0x9b05688c2b3e6c1f,
        0x1f83d9abfb41bd6b, 0x5be0cd19137e2179,
        0xcbbb9d5d66a37e35, 0x629a292a366cd219,
        0x9159015a1e78b8f2, 0x152fecd8f70e5939,
        0x67332667ffc00b31, 0x8eb44a8768581511,
        0xdb0c2e0d64f98fa7, 0x47b5481dbefa4fa4,
    ];

    // test permutation: init state, run one perm
    let mut state = [FE256_ZERO; STATE_SIZE];
    for i in 0..16 { state[i] = Fe256::from_u64(iv[i]); }

    println!("\nState before permute:");
    for i in 0..4 {
        println!("  state[{}] = {:016x} {:016x} {:016x} {:016x}", i, state[i].0[3], state[i].0[2], state[i].0[1], state[i].0[0]);
    }

    permute256(&mut state);

    println!("\nState after permute:");
    for i in 0..4 {
        println!("  state[{}] = {:016x} {:016x} {:016x} {:016x}", i, state[i].0[3], state[i].0[2], state[i].0[1], state[i].0[0]);
    }
}
