use serde::{Deserialize, Serialize};
use diesel::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = crate::repository::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    #[serde(default)]
    pub id: i32,
    pub username: String,
    pub mobile: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub login_at: Option<chrono::NaiveDateTime>,
}