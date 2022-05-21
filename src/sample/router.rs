use rocket;
use crate::connection;
use crate::sample;
pub fn create_routes() {
    rocket::ignite()
        .manage(connection::init_pool())
        .mount("/comments",
               routes![
                    sample::handler::all_comments,
                    sample::handler::create_comment,
                    sample::handler::get_comment,
                    sample::handler::update_comment,
                    sample::handler::delete_comment
                    ],
        ).launch();
}
