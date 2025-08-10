use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub name: String,
    pub age: i32,
}

#[derive(Serialize, Deserialize, diesel::Queryable, diesel::Selectable)]
#[diesel(table_name = crate::schema::users)]
pub struct DbUser {
    pub id: i32,
    pub name: String,
    pub age: i32,
}
