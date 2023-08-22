// @generated automatically by Diesel CLI.

diesel::table! {
    geojsons (id) {
        id -> Int4,
        name -> Text,
        geojson_data -> Jsonb,
    }
}
