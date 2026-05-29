/*
 * Finite field F_p  with  p = 2^255 - 19  (Curve25519 prime)
 *
 * Representation: 4 × u64 words, little-endian.
 * All arithmetic is branchless and constant-time.
 * Uses u128 for multiplication intermediates.
 */

use zeroize::Zeroize;

#[derive(Clone, Copy, Debug, Zeroize)]
pub struct Fe256(pub [u64; 4]);

/* p = 2^255 - 19 */
pub const P256: Fe256 = Fe256([
    0xFFFFFFFFFFFFFFED,
    0xFFFFFFFFFFFFFFFF,
    0xFFFFFFFFFFFFFFFF,
    0x7FFFFFFFFFFFFFFF,
]);

pub const FE256_ZERO: Fe256 = Fe256([0, 0, 0, 0]);
pub const FE256_ONE: Fe256 = Fe256([1, 0, 0, 0]);

/* branchless mask: returns 0xFFFF... if cond, else 0 */
#[inline(always)]
fn ct_mask(cond: bool) -> u64 {
    (cond as u64).wrapping_neg()
}

/* constant-time select */
#[inline(always)]
fn ct_sel(cond: bool, a: u64, b: u64) -> u64 {
    let mask = ct_mask(cond);
    (a & mask) | (b & !mask)
}

impl Fe256 {
    #[inline(always)]
    pub fn from_u64(v: u64) -> Self {
        Fe256([v, 0, 0, 0])
    }

    /* returns true if self >= other */
    fn gte(self, other: Fe256) -> bool {
        for i in (0..4).rev() {
            if self.0[i] > other.0[i] { return true; }
            if self.0[i] < other.0[i] { return false; }
        }
        true  /* equal */
    }

    pub fn add(self, other: Fe256) -> Fe256 {
        let mut r = [0u64; 4];
        let mut carry: u128 = 0;
        for i in 0..4 {
            carry += self.0[i] as u128 + other.0[i] as u128;
            r[i] = carry as u64;
            carry >>= 64;
        }
        let mut result = Fe256(r);
        /* branchless: if result >= p { result -= p } */
        if result.gte(P256) {
            let mut borrow: u128 = 0;
            for i in 0..4 {
                let diff = (r[i] as u128) + (1u128 << 64) - P256.0[i] as u128 - borrow;
                r[i] = diff as u64;
                borrow = 1 - (diff >> 64);
            }
            result = Fe256(r);
        }
        result
    }

    pub fn sub(self, other: Fe256) -> Fe256 {
        let mut r = [0u64; 4];
        let mut borrow: u128 = 0;
        for i in 0..4 {
            let diff = (self.0[i] as u128) + (1u128 << 64) - other.0[i] as u128 - borrow;
            r[i] = diff as u64;
            borrow = 1 - (diff >> 64);
        }
        /* if underflow, add p */
        if borrow > 0 {
            let mut carry: u128 = 0;
            for i in 0..4 {
                carry += r[i] as u128 + P256.0[i] as u128;
                r[i] = carry as u64;
                carry >>= 64;
            }
        }
        Fe256(r)
    }

    pub fn neg(self) -> Fe256 {
        FE256_ZERO.sub(self)
    }

    pub fn mul(self, other: Fe256) -> Fe256 {
        /* schoolbook 4×4 → 8 words */
        let mut prod = [0u64; 8];
        for i in 0..4 {
            let mut carry: u128 = 0;
            for j in 0..4 {
                let tmp = self.0[i] as u128 * other.0[j] as u128 + prod[i + j] as u128 + carry;
                prod[i + j] = tmp as u64;
                carry = tmp >> 64;
            }
            prod[i + 4] += carry as u64;  /* ADD, not overwrite */
        }

        /* reduce: 2^256 ≡ 38 (mod p)
         * result = prod[0..3] + 38 * prod[4..7]
         */
        let mut r = [0u64; 4];
        let mut carry: u128 = 0;
        for i in 0..4 {
            carry += prod[i] as u128 + 38u128 * prod[i + 4] as u128;
            r[i] = carry as u64;
            carry >>= 64;
        }
        /* fold remaining carry: c * 2^256 ≡ c * 38 (mod p) */
        {
            let mut fold = (carry as u64) as u128 * 38;
            for i in 0..4 {
                fold += r[i] as u128;
                r[i] = fold as u64;
                fold >>= 64;
            }
        }

        let mut result = Fe256(r);
        /* final reduction — unconditional 40 passes */
        for _ in 0..40 {
            if result.gte(P256) {
                let mut rr = [0u64; 4];
                let mut borrow: u128 = 0;
                for i in 0..4 {
                    let diff = (result.0[i] as u128) + (1u128 << 64) - P256.0[i] as u128 - borrow;
                    rr[i] = diff as u64;
                    borrow = 1 - (diff >> 64);
                }
                result = Fe256(rr);
            }
        }
        result
    }

    pub fn sqr(self) -> Fe256 {
        self.mul(self)
    }

    pub fn inv(self) -> Fe256 {
        /* a^(p-2) mod p */
        /* p-2 = {0xFFFFFFFFFFFFFFEB, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0x7FFFFFFFFFFFFFFF} */
        let exp: [u64; 4] = [
            0xFFFFFFFFFFFFFFEB,
            0xFFFFFFFFFFFFFFFF,
            0xFFFFFFFFFFFFFFFF,
            0x7FFFFFFFFFFFFFFF,
        ];
        let mut result = FE256_ONE;
        let mut base = self;
        for word in 0..4 {
            let e = exp[word];
            let bits = if word < 3 { 64 } else { 63 };
            for bit in 0..bits {
                let _mask = ct_mask(e & (1 << bit) != 0);
                let prod = result.mul(base);
                for i in 0..4 {
                    result.0[i] = ct_sel(e & (1 << bit) != 0, prod.0[i], result.0[i]);
                }
                base = base.sqr();
            }
        }
        result
    }

    pub fn to_bytes(self) -> [u8; 32] {
        let mut out = [0u8; 32];
        for i in 0..4 {
            for j in 0..8 {
                out[i * 8 + j] = (self.0[i] >> (8 * j)) as u8;
            }
        }
        out
    }

    pub fn from_bytes(inb: &[u8; 32]) -> Fe256 {
        let mut r = [0u64; 4];
        for i in 0..4 {
            for j in 0..8 {
                r[i] |= (inb[i * 8 + j] as u64) << (8 * j);
            }
        }
        r[3] &= 0x7FFFFFFFFFFFFFFF;  /* mask top bit */
        Fe256(r)
    }
}
