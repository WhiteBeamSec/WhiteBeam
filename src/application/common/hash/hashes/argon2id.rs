use argon2::PasswordHasher;
use rand::RngCore;
#[macro_use]
build_hash! { ARGON2ID (reader, salt_opt) {
    let mut password: String = String::new();
    reader.read_to_string(&mut password).expect("WhiteBeam: Could not read password buffer");
    let salt: String = match salt_opt {
        Some(val) => val,
        None => {
            let mut rng = rand::thread_rng();
            let mut bytes = [0u8; argon2::password_hash::Salt::RECOMMENDED_LENGTH];
            rng.fill_bytes(&mut bytes);
            String::from(argon2::password_hash::SaltString::b64_encode(&bytes).expect("WhiteBeam: Salt string invariant violated").as_str())
        }
    };
    // Argon2 with default params (Argon2id v19)
    let argon2 = argon2::Argon2::default();
    // Hash password to PHC string ($argon2id$v=19$...)
    argon2.hash_password_simple::<String>(password.as_bytes(), &salt).unwrap().to_string()
}}
