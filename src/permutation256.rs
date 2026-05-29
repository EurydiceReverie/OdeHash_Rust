/*
 * ODE-based permutation — 256-bit field version.
 * 8 rounds, 16 Taylor steps each, Feistel feedback.
 */

use crate::field256::*;
use zeroize::Zeroize;

pub const STATE_SIZE: usize = 16;
pub const RATE: usize = 8;
pub const ROUNDS: usize = 8;
pub const STEPS_PER_ROUND: usize = 16;

const BASE_RC: [u64; 8] = [
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a,
    0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
];

fn rc(round: usize, index: usize) -> Fe256 {
    Fe256::from_u64(BASE_RC[index] + round as u64)
}

pub fn permute256(state: &mut [Fe256; STATE_SIZE]) {
    let mut old = *state;

    for round in 0..ROUNDS {
        // 1. round constants
        for i in 0..8 {
            state[i] = state[i].add(rc(round, i));
        }

        // 2. polynomial coefficients from all 16 state words
        let mut p1 = [FE256_ZERO; 8];
        let mut q1 = [FE256_ZERO; 8];
        let mut r1 = [FE256_ZERO; 8];
        let mut c1 = [FE256_ZERO; 8];
        let mut p2 = [FE256_ZERO; 8];
        let mut q2 = [FE256_ZERO; 8];
        let mut r2 = [FE256_ZERO; 8];
        let mut c2 = [FE256_ZERO; 8];

        for i in 0..8 {
            let s = state[i];
            let t = state[(i + 8) % 16];

            p1[i] = s.add(t.mul(Fe256::from_u64(0x9e3779b9)));
            q1[i] = s.add(t).mul(Fe256::from_u64(0x517cc1b7));
            r1[i] = s.add(t.mul(Fe256::from_u64(0x85ebca6b))).sqr();
            c1[i] = s.mul(t.add(Fe256::from_u64(0xc2b2ae35)));

            p2[i] = s.add(t).mul(Fe256::from_u64(0x27d4eb2f));
            q2[i] = s.mul(t.add(Fe256::from_u64(0x165667b1)));
            r2[i] = s.mul(Fe256::from_u64(0x9b05688c)).add(t).sqr();
            c2[i] = s.mul(t).add(Fe256::from_u64(0xa54ff53a));
        }

        // 3. initial conditions
        let mut a = [FE256_ZERO; STEPS_PER_ROUND + 1];
        let mut b = [FE256_ZERO; STEPS_PER_ROUND + 1];
        a[0] = state[8].add(state[10].mul(state[11]));
        b[0] = state[9].add(state[12].mul(state[13]));
        let mix_a = state[14];
        let mix_b = state[15];

        // pre-compute inverses
        let mut inv = [FE256_ZERO; STEPS_PER_ROUND + 1];
        for i in 1..=STEPS_PER_ROUND {
            inv[i] = Fe256::from_u64(i as u64).inv();
        }

        // 4. Taylor recurrence
        for n in 0..STEPS_PER_ROUND {
            let kmax = n.min(7);

            let mut sum_a = p1[n % 8];
            for k in 0..=kmax {
                sum_a = sum_a.add(q1[k].mul(a[n - k]));
                let mut inner_r = FE256_ZERO;
                for j in 0..=n - k {
                    inner_r = inner_r.add(a[j].mul(a[n - k - j]));
                }
                sum_a = sum_a.add(r1[k].mul(inner_r));
                let mut inner_c = FE256_ZERO;
                for j in 0..=n - k {
                    inner_c = inner_c.add(a[j].mul(b[n - k - j]));
                }
                sum_a = sum_a.add(c1[k].mul(inner_c));
            }
            sum_a = sum_a.add(mix_a.mul(Fe256::from_u64((n + 1) as u64)));
            a[n + 1] = sum_a.mul(inv[n + 1]);

            let mut sum_b = p2[n % 8];
            for k in 0..=kmax {
                sum_b = sum_b.add(q2[k].mul(b[n - k]));
                let mut inner_r = FE256_ZERO;
                for j in 0..=n - k {
                    inner_r = inner_r.add(b[j].mul(b[n - k - j]));
                }
                sum_b = sum_b.add(r2[k].mul(inner_r));
                let mut inner_c = FE256_ZERO;
                for j in 0..=n - k {
                    inner_c = inner_c.add(a[j].mul(b[n - k - j]));
                }
                sum_b = sum_b.add(c2[k].mul(inner_c));
            }
            sum_b = sum_b.add(mix_b.mul(Fe256::from_u64((n + 1) as u64)));
            b[n + 1] = sum_b.mul(inv[n + 1]);
        }

        // 5. Feistel feedback
        for i in 0..8 {
            state[i] = a[9 + i].add(old[i]);
            state[8 + i] = b[9 + i].add(old[8 + i]);
        }

        old = *state;

        a.zeroize();
        b.zeroize();
    }

    old.zeroize();
}

pub fn absorb256(state: &mut [Fe256; STATE_SIZE], block: &[Fe256; RATE]) {
    for i in 0..RATE {
        state[i] = state[i].add(block[i]);
    }
    permute256(state);
}
