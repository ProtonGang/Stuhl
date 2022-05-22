#![allow(proc_macro_derive_resolution_fallback)]

use chrono::NaiveDateTime as Time;
use crate::schema::comments;

#[derive(Queryable, Serialize, AsChangeset, Deserialize, Debug)]
#[table_name = "comments"]
pub struct Comment {
    pub id: i32,
    pub body: String,
    pub user_id: i32,
    pub post_url: String,
    pub approved: bool,
    pub date: Time,
    pub last_edit: Option<Time>
}
#[derive(Insertable, Serialize, Deserialize)]
#[table_name="comments"]
pub struct NewComment {
    pub user_id: i32,
    pub body: String,
}
