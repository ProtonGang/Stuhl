#![allow(proc_macro_derive_resolution_fallback)]
use diesel;
use diesel::prelude::*;
use crate::sample::model::Comment;
use crate::sample::model::NewComment;
use crate::schema::comments;
use crate::schema::comments::dsl::*;

pub fn create_comment(new_comment: NewComment, conn: &PgConnection) -> QueryResult<Comment> {
    diesel::insert_into(comments::table)
        .values(&new_comment)
        .get_result(conn)
}

pub fn show_comments(connection: &PgConnection) -> QueryResult<Vec<Comment>>  {
    //comments.filter(published.eq(true))
    comments.limit(5)
        .load::<Comment>(&*connection)
}

pub fn get_comment(comment_id: i32, connection: &PgConnection) -> QueryResult<Comment> {
    comments::table.find(comment_id).get_result::<Comment>(connection)
}

pub fn update_comment(comment_id: i32, comment: Comment, connection: &PgConnection) -> QueryResult<Comment> {
    diesel::update(comments::table.find(comment_id))
        .set(&comment)
        .get_result(connection)
}

pub fn delete_comment(comment_id: i32, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(comments::table.find(comment_id))
        .execute(connection)
}
