use serde::{Deserialize, Serialize};

// use crate::{
//     //config::db::Connection,
//     util::constant,
//     // models::{login_history::LoginHistory, user_token::UserToken},
//     //schema::users::{self, dsl::*},
// };
// use bcrypt::{hash, verify, DEFAULT_COST};
// use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    // pub login_session: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserDTO {
    pub username: String,
    pub email: String,
    pub password: String,
}

// #[derive(Serialize, Deserialize)]
// pub struct LoginDTO {
//     pub username_or_email: String,
//     pub password: String,
// }

#[derive(Serialize, Deserialize)]
pub struct LoginInfoDTO {
    pub username: String,
    // pub login_session: String,
}