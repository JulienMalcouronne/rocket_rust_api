use rocket::http::Status;
use rocket::response::status::{Custom, NoContent};
use rocket::serde::json::{serde_json::json, Json, Value};

use crate::models::{organisation::NewOrganisation, organisation::Organisation, User};

use crate::repositories::organisations::OrganisationRepository;
use crate::rocket_routes::{server_error, DbConn, EditorUser};

#[rocket::get("/organisations")]
pub async fn get_organisations(db: DbConn, _user: User) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        OrganisationRepository::find_multiple(c, 100)
            .map(|organisations| json!(organisations))
            .map_err(|e| server_error(e.into()))
    })
    .await
}
#[rocket::get("/organisations/<id>")]
pub async fn view_organisation(id: i32, db: DbConn, _user: User) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        OrganisationRepository::find(c, id)
            .map(|organisation| json!(organisation))
            .map_err(|e| server_error(e.into()))
    })
    .await
}
#[rocket::post("/organisations", format = "json", data = "<new_organisation>")]
pub async fn create_organisation(
    new_organisation: Json<NewOrganisation>,
    db: DbConn,
    _user: EditorUser,
) -> Result<Custom<Value>, Custom<Value>> {
    db.run(move |c| {
        OrganisationRepository::create(c, new_organisation.into_inner())
            .map(|organisation| Custom(Status::Created, json!(organisation)))
            .map_err(|e| server_error(e.into()))
    })
    .await
}
#[rocket::put("/organisations/<id>", format = "json", data = "<organisation>")]
pub async fn update_organisation(
    id: i32,
    organisation: Json<Organisation>,
    db: DbConn,
    _user: EditorUser,
) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        OrganisationRepository::update(c, id, organisation.into_inner())
            .map(|organisation| json!(organisation))
            .map_err(|e| server_error(e.into()))
    })
    .await
}
#[rocket::delete("/organisations/<id>")]
pub async fn delete_organisation(
    id: i32,
    db: DbConn,
    _user: EditorUser,
) -> Result<NoContent, Custom<Value>> {
    db.run(move |c| {
        OrganisationRepository::delete(c, id)
            .map(|_| NoContent)
            .map_err(|e| server_error(e.into()))
    })
    .await
}
