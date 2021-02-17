#![allow(proc_macro_derive_resolution_fallback)]
use super::schema::burgers;

pub mod handler;
pub mod repository;
pub mod router;

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "burgers"]
pub struct Burger {
    pub id: i32,
    pub name: String,
    pub description: String,
}
