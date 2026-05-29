use ode_hash_v4::ode_hash_v5_hex;

fn main() {
    let inputs = vec!["", "a", "abc", "hello", "Hello, World!"];
    for i in inputs {
        let h = ode_hash_v5_hex(i.as_bytes());
        println!("  \"{}\" -> {}", i, h);
    }
}
