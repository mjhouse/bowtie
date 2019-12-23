#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] 
extern crate diesel;
extern crate rocket;

extern crate r2d2;
extern crate r2d2_diesel;

// public modules in this lib
pub mod schema;
pub mod models;
pub mod queries;

// private modules in lib
mod database;

// re-exported structs/meathods
pub use database::Conn;