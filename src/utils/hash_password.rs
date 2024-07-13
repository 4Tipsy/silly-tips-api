
use argon2::Argon2;
use argon2::password_hash::{SaltString, PasswordHash, rand_core::OsRng};
use argon2::password_hash::{PasswordHasher, PasswordVerifier}; // ?
use once_cell::sync::Lazy;

use argon2::Algorithm::Argon2id;
use argon2::Version;
use argon2::ParamsBuilder;

// modules
use crate::CONFIG;




static ARGON2: Lazy<Argon2> = Lazy::new(|| {

  let secret = CONFIG.psw_secret.as_bytes();
  let params = ParamsBuilder::new();

  Argon2::new_with_secret(
    secret,
    Argon2id,
    Version::V0x13,
    params.build().unwrap()
  ).unwrap()

});







pub fn get_hashed_psw(psw: &str) -> String {

  let salt = SaltString::generate(&mut OsRng);
  ARGON2.hash_password(psw.as_bytes(), &salt).unwrap().to_string()
}


pub fn verify_password(psw: &str, hashed_psw_from_db: &str) -> bool {

  let parsed_hashed_psw = PasswordHash::new( &hashed_psw_from_db ).unwrap();
  ARGON2.verify_password(psw.as_bytes(), &parsed_hashed_psw).is_ok()
}