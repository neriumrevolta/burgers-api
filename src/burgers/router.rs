use crate::burgers;
use crate::connection;
use rocket;

pub fn create_routes() {
    rocket::ignite()
        .manage(connection::init_pool())
        .mount(
            "/burgers",
            routes![
                burgers::handler::all,
                burgers::handler::get,
                burgers::handler::post,
                burgers::handler::put,
                burgers::handler::delete
            ],
        )
        .launch();
}
