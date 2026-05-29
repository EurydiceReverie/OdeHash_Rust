use ode_hash_v4::ode_hash;
use std::time::Instant;

fn main() {
    println!("=== ODE-Hash v4 Benchmark (Rust) ===\n");

    let tests: &[(usize, usize, &str)] = &[
        (32, 100, "    32 B"),
        (64, 100, "    64 B"),
        (128, 50, "   128 B"),
        (1024, 20, "  1024 B"),
        (4096, 10, "  4096 B"),
        (65536, 2, "  64 KiB"),
    ];

    println!("  {:<10}  {:>5}  {:>12}  {:>10}", "Size", "Iters", "Bytes/sec", "ms/hash");
    println!("  {:<10}  {:>5}  {:>12}  {:>10}", "----", "-----", "---------", "-------");

    for &(size, iters, label) in tests {
        let input: Vec<u8> = (0..size).map(|_| rand::random::<u8>()).collect();

        let start = Instant::now();
        for _ in 0..iters {
            let _ = ode_hash(&input);
        }
        let elapsed = start.elapsed().as_secs_f64();
        let bytes_per_sec = size as f64 * iters as f64 / elapsed;
        let ms_per_hash = elapsed / iters as f64 * 1000.0;

        println!("  {:<10}  {:>5}  {:>12.0}  {:>10.3}", label, iters, bytes_per_sec, ms_per_hash);
    }
    println!();
}
