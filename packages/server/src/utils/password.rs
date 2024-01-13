use bcrypt::{hash, verify, BcryptError, DEFAULT_COST};
use std::result::Result;

pub fn gen_hash(password: &str) -> Result<String, BcryptError> {
    let hashed = hash(password, DEFAULT_COST);

    hashed
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, BcryptError> {
    let is_valid = verify(password, hash);

    is_valid
}
