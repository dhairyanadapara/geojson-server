// @generated automatically by Diesel CLI.

diesel::table! {
    geojsons (id) {
        id -> Int4,
        name -> Text,
        geojson_data -> Jsonb,
    }
}

diesel::table! {
    spatial_ref_sys (srid) {
        srid -> Int4,
        #[max_length = 256]
        auth_name -> Nullable<Varchar>,
        auth_srid -> Nullable<Int4>,
        #[max_length = 2048]
        srtext -> Nullable<Varchar>,
        #[max_length = 2048]
        proj4text -> Nullable<Varchar>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(geojsons, spatial_ref_sys,);
