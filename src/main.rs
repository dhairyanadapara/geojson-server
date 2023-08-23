#[macro_use]
extern crate rocket;

// use crate::routes;
use dotenv::dotenv;
use rocket::routes;

mod routes;

#[rocket::main]
async fn main() {
    dotenv().ok();

    match rocket::build()
        .mount(
            "/api",
            routes![
                crate::routes::health_check::health_check,
                crate::routes::geojson::get_geojson,
                crate::routes::geojson::add_geojson,
            ],
        )
        .launch()
        .await
    {
        Ok(_) => println!("Server started"),
        Err(e) => println!("Failed to start server {:?}", e),
    }
}
