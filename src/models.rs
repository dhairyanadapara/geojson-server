use diesel::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::geojsons)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct GeoJSONData {
    pub id: i32,
    pub name: String,
    pub geojson_data: serde_json::Value,
}

#[derive(Deserialize, Insertable, Serialize)]
#[diesel(table_name = crate::schema::geojsons)]
pub struct NewGeoJSONData {
    pub name: String,
    pub geojson_data: serde_json::Value,
}

#[derive(Deserialize, Serialize)]
pub struct GeoJSONList {
    pub geojson_list: Vec<GeoJSONData>,
}
