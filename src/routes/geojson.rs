use diesel::sql_query;
use diesel::RunQueryDsl;
use geojson_server::database::establish_connection;
use geojson_server::models::GeoJSONData;
use geojson_server::models::GeoJSONList;
use geojson_server::models::NewGeoJSONData;
use geojson_server::schema::geojsons::dsl::*;
use rocket::response::status::NoContent;
use rocket::serde::json::Json;
use rocket::{get, http::Status};

#[get("/geojsons")]
pub fn get_geojson() -> Json<GeoJSONList> {
    let connection = &mut establish_connection();

    let geojson_list: Vec<GeoJSONData> = geojsons.load(connection).expect("Error loading geojsons");

    Json(GeoJSONList { geojson_list })
}

// handle preflight check for CORS request
#[options("/geojsons")]
pub fn geojsons_preflight() -> NoContent {
    NoContent
}

#[post("/geojsons", format = "json", data = "<geojson>")]
pub fn add_geojson(geojson: Json<NewGeoJSONData>) -> Status {
    let connection = &mut establish_connection();

    let _ = diesel::insert_into(geojsons)
        .values(&*geojson)
        .execute(connection);

    Status::Ok
}

#[post("/geojson_by_viewport", format = "json", data = "<viewport>")]
pub fn get_geojson_list_by_view_geojson(viewport: Json<serde_json::Value>) -> Json<GeoJSONList> {
    let connection = &mut establish_connection();

    let query = sql_query(format!(
        "SELECT
            id, name, geojson_data
        FROM 
            geojsons
        WHERE EXISTS (
            SELECT 1
            FROM jsonb_array_elements(geojson_data->'features') AS feature
            WHERE
                ST_Within(ST_GeomFromGeoJSON(feature->>'geometry'), ST_GeomFromGeoJSON('{}'))
        )
        ",
        // AND
        //         ST_Area(ST_GeomFromGeoJSON(feature->>'geometry')) < ST_Area(ST_GeomFromGeoJSON('{}')))
        // WHERE ST_Contains(
        //     ST_GeomFromGeoJSON('{}'),
        //     ST_GeomFromGeoJSON(feature->>'geometry')
        // AND
        //
        // )
        viewport.to_string()
    ));

    let results = query.load::<GeoJSONData>(connection).unwrap();

    Json(GeoJSONList {
        geojson_list: results.iter().cloned().collect(),
    })
}

// handle preflight check for CORS request
#[options("/geojson_by_viewport")]
pub fn get_geojson_list_by_view_geojson_preflight() -> NoContent {
    NoContent
}

#[delete("/geojson/<gid>")]
pub fn delete_geojson(gid: String) -> NoContent {
    let connection = &mut establish_connection();

    let query = sql_query(format!(
        "DELETE FROM
            geojsons
        WHERE 
            id={}
        ",
        gid
    ));

    query.execute(connection).unwrap();

    NoContent
}
