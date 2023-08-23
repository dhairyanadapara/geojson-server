use diesel::RunQueryDsl;
use geojson_server::database::establish_connection;
use geojson_server::models::GeoJSONData;
use geojson_server::models::GeoJSONList;
use geojson_server::models::NewGeoJSONData;
use geojson_server::schema::geojsons::dsl::*;
use rocket::serde::json::Json;
use rocket::{get, http::Status};

#[get("/geojsons")]
pub fn get_geojson() -> Json<GeoJSONList> {
    let connection = &mut establish_connection();

    // let results = select_a
    let geojson_list: Vec<GeoJSONData> = geojsons.load(connection).expect("Error loading geojsons");

    Json(GeoJSONList { geojson_list })
}

#[post("/geojsons", format = "json", data = "<geojson>")]
pub fn add_geojson(geojson: Json<NewGeoJSONData>) -> Status {
    let connection = &mut establish_connection();

    let _ = diesel::insert_into(geojsons)
        .values(&*geojson)
        .execute(connection);

    Status::Ok
}
