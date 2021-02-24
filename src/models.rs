use diesel::{Insertable, Queryable};
use serde::{Serialize, Deserialize};
use super::schema::actix_users;
use uuid::Uuid;
use std::hash::{Hash, Hasher};

#[derive(Insertable, Queryable, Serialize, Deserialize)]
pub struct ActixUser {
    pub uuid: Uuid,
    pub username:String,
    pub password:String
}
#[derive(Serialize, Deserialize, Hash)]
pub struct NewUser {
    pub username:String,
    pub unhashed:String
}
