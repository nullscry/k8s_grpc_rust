use crate::schema::{features, models};
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = models)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Model {
    pub id: i32,
    pub name: String,
    pub category: String,
}

#[derive(Insertable)]
#[diesel(table_name = models)]
pub struct NewModel {
    pub name: String,
    pub category: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = features)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Feature {
    pub id: i32,
    pub name: String,
    pub category: String,
    pub model: String,
}

#[derive(Insertable)]
#[diesel(table_name = features)]
pub struct NewFeature {
    pub name: String,
    pub category: String,
    pub model: String,
}
