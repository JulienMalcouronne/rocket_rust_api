#[macro_use]
extern crate diesel;

pub mod auth;
mod models;
mod repositories;

pub mod commands;
pub mod rocket_routes;
mod schema;
