use std::env;

use diesel::result::Error;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

use crate::connection::DbConn;
use crate::sample;
use crate::sample::model::Comment;
use crate::sample::model::NewComment;

#[get("/")]
pub fn all_comments(connection: DbConn) -> Result<Json<Vec<Comment>>, Status> {
    sample::repository::show_comments(&connection)
        .map(|comment| Json(comment))
        .map_err(|error| error_status(error))
}

#[post("/", format ="application/json", data = "<new_comment>")]
pub fn create_comment(new_comment: Json<NewComment>, connection: DbConn) ->  Result<status::Created<Json<Comment>>, Status> {
    println!("here 0 {}",&new_comment.title);
    sample::repository::create_comment(new_comment.into_inner(), &connection)
        .map(|comment| comment_created(comment))
        .map_err(|error| error_status(error))
}

#[get("/<id>")]
pub fn get_comment(id: i32, connection: DbConn) -> Result<Json<Comment>, Status> {
    sample::repository::get_comment(id, &connection)
        .map(|comment| Json(comment))
        .map_err(|error| error_status(error))
}

#[put("/<id>", format = "application/json", data = "<comment>")]
pub fn update_comment(id: i32, comment: Json<Comment>, connection: DbConn) -> Result<Json<Comment>, Status> {
    sample::repository::update_comment(id, comment.into_inner(), &connection)
        .map(|comment| Json(comment))
        .map_err(|error| error_status(error))
}

#[delete("/<id>")]
pub fn delete_comment(id: i32, connection: DbConn) -> Result<status::NoContent, Status> {
    sample::repository::delete_comment(id, &connection)
        .map(|_| status::NoContent)
        .map_err(|error| error_status(error))
}

fn comment_created(comment: Comment) -> status::Created<Json<Comment>> {
    println!("here final");
    status::Created(
        format!("{host}:{port}/comment/{id}", host = host(), port = port(), id = comment.id).to_string(),
        Some(Json(comment)))
}

fn host() -> String {
    env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set")
}

fn port() -> String {
    env::var("ROCKET_PORT").expect("ROCKET_PORT must be set")
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    }
}
