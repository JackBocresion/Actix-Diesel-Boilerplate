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


pub fn create_user(data: Json<NewUser>) -> ActixUser {
    let salt = b"salsdfsdfsdft";
    let config = Config::default();

    ActixUser {
        username:data.username.to_string(),
        password:xx::hash64(data.unhashed.to_string()).to_string(),
        uuid:Uuid::new_v4()
    }
}
//the hasher below is really slow
//hash_encoded(data.unhashed.as_bytes(), salt, &config).unwrap(),