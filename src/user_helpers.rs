use argon2::{hash_encoded, Config};
use uuid::Uuid;
use super::models::{ActixUser, NewUser};
use actix_web::web::Json;

use std::hash::{Hash, Hasher};

use fasthash::{xx, XXHasher};

fn hash<T: Hash>(t: &T) -> u64 {
    let mut s: XXHasher = Default::default();
    t.hash(&mut s);
    s.finish()
}



