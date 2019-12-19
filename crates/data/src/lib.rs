#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate diesel;

pub mod schema;
pub mod models;
pub mod queries;
pub mod database;
pub mod traits;