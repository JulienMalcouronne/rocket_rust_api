use crate::models::{Crate, NewCrate, User};
use crate::repositories::CrateRepository;
use crate::rocket_routes::{server_error, DbConn};
use rocket::http::Status;
use rocket::response::status::{Custom, NoContent};
use rocket::serde::json::Json;
use rocket::serde::json::{serde_json::json, Value};

#[rocket::get("/crates")]
pub async fn get_crates(db: DbConn, _user: User) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        CrateRepository::find_multiple(c, 100)
            .map(|crates| json!(crates))
            .map_err(|_| Custom(Status::InternalServerError, json!("error")))
    })
    .await
}

#[rocket::get("/crates/<id>")]
pub async fn view_crate(id: i32, db: DbConn, _user: User) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        CrateRepository::find(c, id)
            .map(|a_crate| json!(a_crate))
            .map_err(|e| server_error(e.into()))
    })
    .await
}

#[rocket::post("/crates", format = "json", data = "<new_crate>")]
pub async fn create_crate(
    new_crate: Json<NewCrate>,
    db: DbConn,
    _user: User,
) -> Result<Custom<Value>, Custom<Value>> {
    db.run(move |c| {
        CrateRepository::create(c, new_crate.into_inner())
            .map(|a_crate| Custom(Status::Created, json!(a_crate)))
            .map_err(|e| server_error(e.into()))
    })
    .await
}

#[rocket::put("/crates/<id>", format = "json", data = "<the_crate>")]
pub async fn update_crate(
    id: i32,
    the_crate: Json<Crate>,
    db: DbConn,
    _user: User,
) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        CrateRepository::save(c, id, the_crate.into_inner())
            .map(|a_crate| json!(a_crate))
            .map_err(|e| server_error(e.into()))
    })
    .await
}

#[rocket::delete("/crates/<id>")]
pub async fn delete_crate(id: i32, db: DbConn, _user: User) -> Result<NoContent, Custom<Value>> {
    db.run(move |c| {
        CrateRepository::delete(c, id)
            .map(|_| NoContent)
            .map_err(|e| server_error(e.into()))
    })
    .await
}
