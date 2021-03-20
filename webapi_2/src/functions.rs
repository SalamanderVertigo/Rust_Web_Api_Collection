use argon2::{self, Config};

pub fn generate_hash(password: &[u8]) -> String {
    // let password = b"password";
    let salt = b"randomsalt";
    let config = Config::default();
    let hash = argon2::hash_encoded(password, salt, &config).unwrap();
    println!("hash: {}", hash);
    return hash
    }