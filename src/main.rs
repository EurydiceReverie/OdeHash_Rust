use clap::Parser;
use std::fs;
use std::io::{Read, BufReader};
use std::process;

use ode_hash::sponge256::{Sponge256, ODE_HASH256};

#[derive(Parser)]
#[command(name = "odehash")]
#[command(about = "OdeHash v5: Coupled Riccati ODE Hash (256-bit field)")]
struct Args {
    /// Input string to hash
    #[arg(short, long)]
    string: Option<String>,

    /// Input file to hash
    #[arg(short, long)]
    file: Option<String>,

    /// Show progress bar for file hashing
    #[arg(short)]
    p: bool,

    /// Output "hash  filename" format
    #[arg(long)]
    format: bool,
}

const CHUNK_SIZE: usize = 65536;

fn print_progress(current: u64, total: u64) {
    let pct = if total > 0 { current * 100 / total } else { 0 };
    let bars = pct as usize / 2;
    eprint!("\rHashing: {:3}% [", pct);
    for i in 0..50 {
        if i < bars { eprint!("="); }
        else if i == bars { eprint!(">"); }
        else { eprint!(" "); }
    }
    eprint!("] {}/{} bytes", current, total);
}

fn main() {
    let args = Args::parse();

    // string mode
    if let Some(ref s) = args.string {
        let hash = ode_hash::ode_hash_v5(s.as_bytes());
        for b in &hash { print!("{:02x}", b); }
        println!();
        return;
    }

    // file mode
    let filename = match &args.file {
        Some(f) => f.clone(),
        None => {
            eprintln!("Usage: ode-hash-v5 -s <string>  or  ode-hash-v5 -f <file>");
            process::exit(1);
        }
    };

    let file = match fs::File::open(&filename) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening '{}': {}", filename, e);
            process::exit(1);
        }
    };

    let file_size = file.metadata().map(|m| m.len()).unwrap_or(0);
    let mut reader = BufReader::new(file);
    let mut hasher = Sponge256::new(ODE_HASH256);
    let mut chunk = vec![0u8; CHUNK_SIZE];
    let mut total_read: u64 = 0;

    loop {
        let n = match reader.read(&mut chunk) {
            Ok(0) => break,
            Ok(n) => n,
            Err(e) => {
                eprintln!("\nError reading: {}", e);
                process::exit(1);
            }
        };
        hasher.update(&chunk[..n]);
        total_read += n as u64;
        if args.p { print_progress(total_read, file_size); }
    }

    if args.p { eprintln!(); }

    let hash = hasher.finalize();

    if args.format {
        for b in &hash { print!("{:02x}", b); }
        println!("  {}", filename);
    } else {
        for b in &hash { print!("{:02x}", b); }
        println!();
    }
}
