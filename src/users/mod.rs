use serde::{Deserialize, Serialize};

mod create_user;

pub use create_user::create_user;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
}
