use ode_hash_v4::ode_hash;
use rand::Rng;

fn hamming(a: &[u8], b: &[u8]) -> usize {
    a.iter().zip(b.iter()).map(|(x, y)| (x ^ y).count_ones() as usize).sum()
}

#[test]
fn test_avalanche() {
    let mut rng = rand::thread_rng();
    let n = 100;
    let mut total = 0usize;
    let mut lo = 256usize;
    let mut hi = 0usize;

    for _ in 0..n {
        let mut input = [0u8; 32];
        rng.fill(&mut input);
        let h1 = ode_hash(&input);

        let bit = rng.gen_range(0..256);
        input[bit / 8] ^= 1u8 << (bit % 8);
        let h2 = ode_hash(&input);

        let d = hamming(&h1, &h2);
        total += d;
        lo = lo.min(d);
        hi = hi.max(d);
    }

    let avg = total as f64 / n as f64;
    let pct = avg / 256.0 * 100.0;

    println!("  tests        : {}", n);
    println!("  avg distance : {:.1} / 256  ({:.1}%)", avg, pct);
    println!("  ideal (50%)  : 128.0");
    println!("  min / max    : {} / {}", lo, hi);

    assert!(pct >= 30.0 && pct <= 70.0,
        "avalanche {:.1}% outside [30%, 70%]", pct);
}
