use rocket::{http::Status, response::status::Custom, serde::json::Json};
use rocket_db_pools::{deadpool_redis::redis::AsyncCommands, Connection};
use serde_json::{json, Value};

use crate::{auth, repositories::UserRepository};

use super::{server_error, CacheConn, DbConn};

#[rocket::post("/login", format = "json", data = "<credentials>")]
pub async fn login(
    credentials: Json<auth::Credentials>,
    db: DbConn,
    mut cache: Connection<CacheConn>,
) -> Result<Value, Custom<Value>> {
    let username = credentials.username.clone();
    let user = db
        .run(move |c| {
            UserRepository::find_by_username(c, &username).map_err(|e| server_error(e.into()))
        })
        .await?;
    let session_id = auth::authorize_user(&user, &credentials)
        .map_err(|_| Custom(Status::Unauthorized, json!("Wrong credential")))?;

    cache
        .set_ex::<_, _, ()>(format!("sessions/{}", session_id), user.id, 3 * 60 * 60)
        .await
        .map(|_| json!({ "token": session_id }))
        .map_err(|e| server_error(e.into()))
}
