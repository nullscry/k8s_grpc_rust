// @generated automatically by Diesel CLI.

diesel::table! {
    features (id) {
        id -> Int4,
        name -> Varchar,
        category -> Varchar,
        model -> Varchar,
    }
}

diesel::table! {
    models (id) {
        id -> Int4,
        name -> Varchar,
        category -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(features, models,);
