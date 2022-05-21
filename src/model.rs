#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "comments"]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}
#[derive(Insertable, Serialize, Deserialize)]
#[table_name="comments"]
pub struct NewComment {
    pub title: String,
    pub body: String,
}
