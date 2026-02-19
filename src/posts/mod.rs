use serde::{Deserialize, Serialize};

mod create_post;
mod delete_post;
mod get_post_by_id;
mod get_posts;
mod update_post;

pub use create_post::create_post;
pub use delete_post::delete_post;
pub use get_post_by_id::get_post_by_id;
pub use get_posts::get_posts;
pub use update_post::update_post;

#[derive(Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub user_id: Option<i32>,
    pub title: String,
    pub body: String,
}
