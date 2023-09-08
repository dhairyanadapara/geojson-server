use diesel::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(
    Queryable, Selectable, Insertable, Serialize, Deserialize, Debug, QueryableByName, Clone,
)]
#[diesel(table_name = crate::schema::geojsons)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct GeoJSONData {
    pub id: i32,
    pub name: String,
    pub geojson_data: serde_json::Value,
}

#[derive(Deserialize, Insertable, Serialize, Debug)]
#[diesel(table_name = crate::schema::geojsons)]
pub struct NewGeoJSONData {
    pub name: String,
    pub geojson_data: serde_json::Value,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GeoJSONList {
    pub geojson_list: Vec<GeoJSONData>,
}

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = crate::schema::geojsons)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct GeoJson(serde_json::Value);
