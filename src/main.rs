#[macro_use]
extern crate rocket;

// use crate::routes;
use rocket::routes;
use std::env;

use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;

use dotenv::dotenv;

mod routes;

#[rocket::main]
async fn main() {
    dotenv().ok();

    rocket::build()
        .mount(
            "/api",
            routes![
                crate::routes::health_check::health_check,
                crate::routes::geojson::get_geojson,
                crate::routes::geojson::add_geojson,
            ],
        )
        .launch()
        .await;

    println!("Server started");
}
