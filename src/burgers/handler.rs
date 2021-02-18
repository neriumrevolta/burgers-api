use crate::burgers;
use crate::burgers::Burger;
use crate::burgers::InsertableBurger;
use crate::connection::DbConn;
use diesel::result::Error;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;
use std::env;

#[get("/")]
pub fn all(connection: DbConn) -> Result<Json<Vec<Burger>>, Status> {
    burgers::repository::all(&connection)
        .map(|burgers| Json(burgers))
        .map_err(|error| error_status(error))
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError,
    }
}

#[get("/<id>")]
pub fn get(id: i32, connection: DbConn) -> Result<Json<Burger>, Status> {
    burgers::repository::get(id, &connection)
        .map(|burger| Json(burger))
        .map_err(|error| error_status(error))
}

#[get("/name/<name>")]
pub fn find_by_name(name: String, connection: DbConn) -> Result<Json<Vec<Burger>>, Status> {
    burgers::repository::find_by_name(name, &connection)
        .map(|burger| Json(burger))
        .map_err(|error| error_status(error))
}

#[get("/random")]
pub fn rand(connection: DbConn) -> Result<Json<Burger>, Status> {
    burgers::repository::rand(&connection)
        .map(|burger| Json(burger))
        .map_err(|error| error_status(error))
}

#[post("/", format = "application/json", data = "<burger>")]
pub fn post(
    burger: Json<InsertableBurger>,
    connection: DbConn,
) -> Result<status::Created<Json<Burger>>, Status> {
    burgers::repository::insert(burger.into_inner(), &connection)
        .map(|burger| burger_created(burger))
        .map_err(|error| error_status(error))
}

fn burger_created(burger: Burger) -> status::Created<Json<Burger>> {
    status::Created(
        format!(
            "{host}:{port}/burgers/{id}",
            host = host(),
            port = port(),
            id = burger.id
        )
        .to_string(),
        Some(Json(burger)),
    )
}

fn host() -> String {
    env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set")
}

fn port() -> String {
    env::var("ROCKET_PORT").expect("ROCKET_PORT must be set")
}

#[put("/<id>", format = "application/json", data = "<burger>")]
pub fn put(
    id: i32,
    burger: Json<InsertableBurger>,
    connection: DbConn,
) -> Result<Json<Burger>, Status> {
    burgers::repository::update(id, burger.into_inner(), &connection)
        .map(|burger| Json(burger))
        .map_err(|error| error_status(error))
}

#[delete("/<id>")]
pub fn delete(id: i32, connection: DbConn) -> Result<Status, Status> {
    match burgers::repository::get(id, &connection) {
        Ok(_) => burgers::repository::delete(id, &connection)
            .map(|_| Status::NoContent)
            .map_err(|error| error_status(error)),
        Err(error) => Err(error_status(error)),
    }
}
