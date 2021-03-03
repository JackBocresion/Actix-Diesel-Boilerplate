use diesel::{Insertable, Queryable};
use serde::{Serialize, Deserialize};
use super::schema::actix_users;
use uuid::Uuid;
use argon2::{hash_encoded, Config};
use actix_web::web::Json;
use std::hash::{Hash, Hasher};

use fasthash::{xx, XXHasher};

#[derive(Insertable, Queryable, Serialize, Deserialize)]
pub struct ActixUser {
    pub uuid: Uuid,
    pub username:String,
    pub password:String
}
use std::fmt;
impl fmt::Display for ActixUser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "({} | {}, {})", self.uuid, self.username, self.password)
    }
}

impl ActixUser {
    pub fn new(data:Json<NewUser>) -> ActixUser {
        let salt = b"salsdfsdfsdft";
        let config = Config::default();
        ActixUser{
            username:data.username.to_string(),
            password:xx::hash64(data.unhashed.to_string()).to_string(),
            uuid:Uuid::new_v4()
        }
    }
}
#[derive(Serialize, Deserialize, Hash)]
pub struct NewUser {
    pub username:String,
    pub unhashed:String
}
