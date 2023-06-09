pub mod authorization;
pub mod crates;
pub mod rustaceans;
use diesel::PgConnection;

use rocket::http::Status;

use rocket::response::status::Custom;
use rocket::serde::json::{serde_json::json, Value};
use rocket::Request;

use rocket::request::{FromRequest, Outcome};

use rocket_db_pools::deadpool_redis::redis::AsyncCommands;
use rocket_db_pools::{deadpool_redis, Connection, Database};

use crate::models::User;
use crate::repositories::UserRepository;

#[rocket_sync_db_pools::database("postgres")]
pub struct DbConn(PgConnection);

#[derive(Database)]
#[database("redis")]
pub struct CacheConn(deadpool_redis::Pool);

pub fn server_error(e: Box<dyn std::error::Error>) -> Custom<Value> {
    log::error!("{}", e);
    Custom(Status::InternalServerError, json!("error"))
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let session_header = request
            .headers()
            .get_one("Authorization")
            .map(|v| v.split_whitespace().collect::<Vec<_>>())
            .filter(|v| v.len() == 2 && v[0] == "Bearer");

        if let Some(session_value) = session_header {
            let mut cache = request
                .guard::<Connection<CacheConn>>()
                .await
                .expect("Cannot connect to redis in request guard");
            let db = request
                .guard::<DbConn>()
                .await
                .expect("Cannot connect to postgres in request guard");
            let result = cache
                .get::<_, i32>(format!("sessions/{}", session_value[1]))
                .await;

            if let Ok(user_id) = result {
                return match db.run(move |c| UserRepository::find(c, user_id)).await {
                    Ok(user) => Outcome::Success(user),
                    _ => Outcome::Failure((Status::Unauthorized, ())),
                };
            }
        }

        Outcome::Failure((Status::Unauthorized, ()))
    }
}
