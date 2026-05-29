use ode_hash_v4::ode_hash_hex;

#[test]
fn test_empty() {
    let h = ode_hash_hex(b"");
    assert_eq!(h, "df33ba1c78b8e409ab701f79d4677831e3ee9563ba543c1f62f5fd3c56e20713");
}

#[test]
fn test_a() {
    let h = ode_hash_hex(b"a");
    assert_eq!(h, "2b0e810fa7acc1f35f2401ed0a1466b00a611121a4161ba63870858b03a9b92b");
}

#[test]
fn test_abc() {
    let h = ode_hash_hex(b"abc");
    assert_eq!(h, "ed26f84a364c12d1d4a9eed7bcadb94d25c37ff9bf152ab15296588c51765baa");
}

#[test]
fn test_hello() {
    let h = ode_hash_hex(b"hello");
    assert_eq!(h, "04b63cf0a4b11c2734f338292f062c5568f14dd9154ee9ff15c6a518eab1be71");
}

#[test]
fn test_hello_world() {
    let h = ode_hash_hex(b"Hello, World!");
    assert_eq!(h, "410b3205a7e31962d53aed9a636e9552cca6886c794bd30e8556012b92b7e318");
}

#[test]
fn test_determinism() {
    let h1 = ode_hash_hex(b"determinism");
    let h2 = ode_hash_hex(b"determinism");
    assert_eq!(h1, h2);
}

#[test]
fn test_different() {
    let h1 = ode_hash_hex(b"input-a");
    let h2 = ode_hash_hex(b"input-b");
    assert_ne!(h1, h2);
}
