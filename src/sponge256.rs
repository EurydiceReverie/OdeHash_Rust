/*
 * Sponge construction for ODE-Hash v5 (256-bit field).
 */

use crate::field256::*;
use crate::permutation256::*;
use zeroize::Zeroize;

pub const ODE_HASH256: u64 = 0;
pub const ODE_MAC256: u64 = 1;
pub const ODE_KDF256: u64 = 2;

const IV: [u64; 16] = [
    0x6a09e667f3bcc908, 0xbb67ae8584caa73b,
    0x3c6ef372fe94f82b, 0xa54ff53a5f1d36f1,
    0x510e527fade682d1, 0x9b05688c2b3e6c1f,
    0x1f83d9abfb41bd6b, 0x5be0cd19137e2179,
    0xcbbb9d5d66a37e35, 0x629a292a366cd219,
    0x9159015a1e78b8f2, 0x152fecd8f70e5939,
    0x67332667ffc00b31, 0x8eb44a8768581511,
    0xdb0c2e0d64f98fa7, 0x47b5481dbefa4fa4,
];

fn rot32(x: u32, n: u32) -> u32 { (x << n) | (x >> (32 - n)) }

fn arx_quarter(v: &mut [u32; 16], a: usize, b: usize, c: usize, d: usize) {
    v[a] = v[a].wrapping_add(v[b]); v[d] ^= v[a]; v[d] = rot32(v[d], 16);
    v[c] = v[c].wrapping_add(v[d]); v[b] ^= v[c]; v[b] = rot32(v[b], 12);
    v[a] = v[a].wrapping_add(v[b]); v[d] ^= v[a]; v[d] = rot32(v[d], 8);
    v[c] = v[c].wrapping_add(v[d]); v[b] ^= v[c]; v[b] = rot32(v[b], 7);
}

fn arx_16_rounds(v: &mut [u32; 16]) {
    // Round constants: nothing-up-my-sleeve numbers (first 4 words of SHA-256 IV).
    // Used only to break symmetry — OdeHash does NOT use SHA-256.
    const RC: [u32; 4] = [0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a];
    v[0] ^= RC[0]; v[1] ^= RC[1]; v[2] ^= RC[2]; v[3] ^= RC[3];

    for r in 0u32..12 {
        v[12] ^= r;
        arx_quarter(v, 0, 4, 8, 12);
        arx_quarter(v, 1, 5, 9, 13);
        arx_quarter(v, 2, 6, 10, 14);
        arx_quarter(v, 3, 7, 11, 15);
        arx_quarter(v, 0, 5, 10, 15);
        arx_quarter(v, 1, 6, 11, 12);
        arx_quarter(v, 2, 7, 8, 13);
        arx_quarter(v, 3, 4, 9, 14);
    }
}

/*
 * ARX finalization — uses ALL 256 bits from each field element.
 * 4 passes × 16 words × 12 rounds, XORed together.
 */
fn arx_finalize256(state: &[Fe256; STATE_SIZE]) -> [u8; 32] {
    let mut accum = [0u32; 16];

    for pass in 0..4 {
        let mut v = [0u32; 16];
        for i in 0..16 {
            v[i] = state[i].0[pass] as u32;
        }
        v[15] ^= pass as u32;
        arx_16_rounds(&mut v);
        for i in 0..16 {
            accum[i] ^= v[i];
        }
    }

    let mut out = [0u8; 32];
    for i in 0..8 {
        let bytes = accum[i].to_le_bytes();
        out[4*i..4*i+4].copy_from_slice(&bytes);
    }
    accum.zeroize();
    out
}

fn bytes_to_block256(block: &mut [Fe256; RATE], bytes: &[u8]) {
    for i in 0..RATE {
        let mut b = [0u8; 32];
        b.copy_from_slice(&bytes[i*32..(i+1)*32]);
        block[i] = Fe256::from_bytes(&b);
    }
}

pub struct Sponge256 {
    state: [Fe256; STATE_SIZE],
    buf: Vec<u8>,
    buf_len: usize,
    total_len: usize,
}

impl Sponge256 {
    pub fn new(domain: u64) -> Self {
        let mut state = [FE256_ZERO; STATE_SIZE];
        for i in 0..16 { state[i] = Fe256::from_u64(IV[i]); }
        state[15] = state[15].add(Fe256::from_u64(domain));
        Sponge256 {
            state,
            buf: vec![0u8; RATE * 32],
            buf_len: 0,
            total_len: 0,
        }
    }

    pub fn update(&mut self, data: &[u8]) {
        let block_bytes = RATE * 32;
        self.total_len += data.len();
        let mut offset = 0;

        if self.buf_len > 0 {
            let space = block_bytes - self.buf_len;
            let copy = data.len().min(space);
            self.buf[self.buf_len..self.buf_len + copy].copy_from_slice(&data[..copy]);
            self.buf_len += copy;
            offset = copy;

            if self.buf_len == block_bytes {
                let mut block = [FE256_ZERO; RATE];
                bytes_to_block256(&mut block, &self.buf);
                absorb256(&mut self.state, &block);
                self.buf_len = 0;
            }
        }

        while offset + block_bytes <= data.len() {
            let mut block = [FE256_ZERO; RATE];
            bytes_to_block256(&mut block, &data[offset..offset + block_bytes]);
            absorb256(&mut self.state, &block);
            offset += block_bytes;
        }

        if offset < data.len() {
            let rem = data.len() - offset;
            self.buf[..rem].copy_from_slice(&data[offset..]);
            self.buf_len = rem;
        }
    }

    pub fn finalize(mut self) -> [u8; 32] {
        let block_bytes = RATE * 32;
        let bit_len = (self.total_len as u64) * 8;

        self.buf[self.buf_len] = 0x80;
        self.buf_len += 1;

        if self.buf_len > block_bytes - 8 {
            for i in self.buf_len..block_bytes { self.buf[i] = 0; }
            let mut block = [FE256_ZERO; RATE];
            bytes_to_block256(&mut block, &self.buf);
            absorb256(&mut self.state, &block);
            self.buf_len = 0;
        }

        for i in self.buf_len..block_bytes { self.buf[i] = 0; }
        for i in 0..8 {
            self.buf[block_bytes - 8 + i] = ((bit_len >> (56 - 8 * i)) & 0xFF) as u8;
        }
        let mut block = [FE256_ZERO; RATE];
        bytes_to_block256(&mut block, &self.buf);
        absorb256(&mut self.state, &block);

        permute256(&mut self.state);
        let out = arx_finalize256(&self.state);

        self.state.zeroize();
        self.buf.zeroize();
        out
    }
}

impl Drop for Sponge256 {
    fn drop(&mut self) {
        self.state.zeroize();
        self.buf.zeroize();
    }
}
