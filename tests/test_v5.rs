use ode_hash::ode_hash_v5_hex;

#[test]
fn test_empty() {
    let h = ode_hash_v5_hex(b"");
    assert_eq!(h, "9cac1f68a3a83eeee5422d8d87fcf3e309234196c19505f858856b24ab584544");
}

#[test]
fn test_a() {
    let h = ode_hash_v5_hex(b"a");
    assert_eq!(h, "bb15f94e90d514d958ff89d4c2dd233f2319f19f201cdf8933a1ea89194de5f0");
}

#[test]
fn test_abc() {
    let h = ode_hash_v5_hex(b"abc");
    assert_eq!(h, "fc78a24fbbc41b17ad3a51f53da24cb18511c2c402cbca2038c22a4ba1a49d34");
}

#[test]
fn test_hello() {
    let h = ode_hash_v5_hex(b"hello");
    assert_eq!(h, "1d145847cf586aeb8a868be4074c42d59190904c1f8c8b6dc24e332dc6aad674");
}

#[test]
fn test_hello_world() {
    let h = ode_hash_v5_hex(b"Hello, World!");
    assert_eq!(h, "3da4cf2813dbde54c21886e3a39e15ba3c750fd35bfb4505b30b0df640877b75");
}

#[test]
fn test_quick_fox() {
    let h = ode_hash_v5_hex(b"The quick brown fox jumps over the lazy dog");
    assert_eq!(h, "e3a6571e970023d63b400c817500309a784f7e41b6752b330102b5ce81a7abfa");
}

#[test]
fn test_hex_input() {
    let h = ode_hash_v5_hex(b"0123456789abcdef");
    assert_eq!(h, "caa99f3a2971fb19b9ac8d70f8bbe6b0528489c5993b09678825f2633831262a");
}

#[test]
fn test_a_repeated() {
    let h = ode_hash_v5_hex(b"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");
    assert_eq!(h, "777c4c1ecbe0dcb03f2647b493759f96151323148cf1fc89c874b425b31bf673");
}

#[test]
fn test_x() {
    let h = ode_hash_v5_hex(b"x");
    assert_eq!(h, "54c2365c1c83d5d292e65284c5e28bb49bf139fe6b308085cb3b5afd73e17c97");
}

#[test]
fn test_test123() {
    let h = ode_hash_v5_hex(b"test123");
    assert_eq!(h, "bb3fd667b857780008b7bad5275010f498885334a8e4bd59ade281b86fe30844");
}

#[test]
fn test_determinism() {
    let h1 = ode_hash_v5_hex(b"determinism");
    let h2 = ode_hash_v5_hex(b"determinism");
    assert_eq!(h1, h2);
}

#[test]
fn test_different() {
    let h1 = ode_hash_v5_hex(b"input-a");
    let h2 = ode_hash_v5_hex(b"input-b");
    assert_ne!(h1, h2);
}
