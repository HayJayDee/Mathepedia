use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::theorems)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Theorem {
    pub id: i32,
    pub title: String,
    pub body: String
}