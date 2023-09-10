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
        .attach(Cors)
        .mount(
            "/api",
            routes![
                crate::routes::health_check::health_check,
                crate::routes::geojson::get_geojson,
                crate::routes::geojson::add_geojson,
                crate::routes::geojson::geojsons_preflight,
                crate::routes::geojson::get_geojson_list_by_view_geojson,
                crate::routes::geojson::get_geojson_list_by_view_geojson_preflight,
                crate::routes::geojson::delete_geojson
            ],
        )
        .launch()
        .await
    {
        Ok(_) => println!("Server started"),
        Err(e) => println!("Failed to start server {:?}", e),
    }
}

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};

pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "CORS",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _req: &'r Request<'_>, res: &mut Response<'r>) {
        res.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        res.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "GET, POST, PUT, DELETE",
        )); // Define allowed methods.
        res.set_header(Header::new("Access-Control-Allow-Headers", "*"));
    }
}
