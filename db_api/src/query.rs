use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::env;

use crate::models::{Feature, Model, NewFeature, NewModel};

use crate::dbapi::{
    InsertFeatureRequest, InsertModelRequest, SelectFeatureRequest, SelectModelRequest,
};

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_model(model_request: InsertModelRequest) -> Model {
    use crate::schema::models;
    let conn = &mut establish_connection();
    let new_row = NewModel {
        name: model_request.name,
        category: model_request.category,
    };

    diesel::insert_into(models::table)
        .values(&new_row)
        .returning(Model::as_returning())
        .get_result(conn)
        .expect("Error saving new model")
}

pub fn create_feature(feature_request: InsertFeatureRequest) -> Feature {
    use crate::schema::features;
    let conn = &mut establish_connection();
    let new_row = NewFeature {
        name: feature_request.name,
        category: feature_request.category,
        model: feature_request.model,
    };

    diesel::insert_into(features::table)
        .values(&new_row)
        .returning(Feature::as_returning())
        .get_result(conn)
        .expect("Error saving new feature")
}

pub fn select_models(select_model: SelectModelRequest) -> Vec<Model> {
    use crate::schema::models;
    use crate::schema::models::dsl::models as query_models;
    let conn = &mut establish_connection();
    let mut query = query_models.into_boxed();
    if select_model.id > 0 {
        query = query.filter(models::id.eq(select_model.id));
    }
    if !select_model.name.is_empty() {
        query = query.filter(models::name.eq(select_model.name.to_string()));
    }
    if !select_model.category.is_empty() {
        query = query.filter(models::category.eq(select_model.category.to_string()));
    }

    let results = query
        .select(Model::as_select())
        .load(conn)
        .optional()
        .unwrap();

    match results {
        Some(results) => results,
        None => vec![],
    }
}

pub fn select_features(select_feature: SelectFeatureRequest) -> Vec<Feature> {
    use crate::schema::features;
    use crate::schema::features::dsl::features as query_features;
    let conn = &mut establish_connection();

    let mut query = query_features.into_boxed();
    if select_feature.id > 0 {
        query = query.filter(features::id.eq(select_feature.id));
    }
    if !select_feature.name.is_empty() {
        query = query.filter(features::name.eq(select_feature.name.to_string()));
    }
    if !select_feature.category.is_empty() {
        query = query.filter(features::category.eq(select_feature.category.to_string()));
    }
    if !select_feature.model.is_empty() {
        query = query.filter(features::model.eq(select_feature.model.to_string()));
    }

    let results = query
        .select(Feature::as_select())
        .load(conn)
        .optional()
        .unwrap();


    match results {
        Some(results) => results,
        None => vec![],
    }
}
