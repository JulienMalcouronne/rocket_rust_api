#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;

mod models;
mod repositories;
mod rocket_routes;
mod schema;
#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/",
            routes![
                rocket_routes::rustaceans::get_rustacean,
                rocket_routes::rustaceans::view_rustacean,
                rocket_routes::rustaceans::create_rustaceans,
                rocket_routes::rustaceans::update_rustacean,
                rocket_routes::rustaceans::delete_rustacean,
            ],
        )
        .launch()
        .await;
}
