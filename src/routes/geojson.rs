use diesel::{
    connection, insert_or_ignore_into, IntoSql, PgConnection, QueryDsl, QuerySource, RunQueryDsl,
};
use geojson_server::database::establish_connection;
use geojson_server::models::GeoJSONData;
use geojson_server::models::NewGeoJSONData;
use geojson_server::schema::geojsons::dsl::*;
use geojson_server::schema::geojsons::geojson_data;
use rocket::http::hyper::server::conn;
use rocket::serde::json::{json, Json, Value};
use rocket::serde::{Deserialize, Serialize};
use rocket::State;
use rocket::{get, http::Status};

#[get("/geojsons")]
pub fn get_geojson() -> Status {
    let connection = &mut establish_connection();

    // let results = select_a
    let all_users: Vec<GeoJSONData> = geojsons.load(connection).expect("Error loading geojsons");

    Status::Ok
}

#[post("/geojsons", format = "json", data = "<geojson>")]
pub fn add_geojson(geojson: Json<NewGeoJSONData>) -> Status {
    let connection = &mut establish_connection();

    diesel::insert_into(geojsons)
        .values(&*geojson)
        .execute(connection);

    Status::Ok
}
