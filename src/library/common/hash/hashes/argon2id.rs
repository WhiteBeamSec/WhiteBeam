use argon2::PasswordHasher;
#[macro_use]
build_hash! { ARGON2ID (reader, salt_opt) {
    let mut password: String = String::new();
    reader.read_to_string(&mut password).expect("WhiteBeam: Could not read password buffer");
    assert!(salt_opt.is_some());
    let salt: String = salt_opt.unwrap();
    // Argon2 with default params (Argon2id v19)
    let argon2 = argon2::Argon2::default();
    // Hash password to PHC string ($argon2id$v=19$...)
    argon2.hash_password_simple(password.as_bytes(), salt.as_ref()).unwrap().to_string()
}}
