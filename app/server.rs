extern crate iron;

#[macro_use]
extern crate router;

extern crate rustc_serialize;

use iron::prelude::*;

pub mod config;
pub mod controllers;
pub mod models;

fn main() {
    Iron::new(config::routes::routes()).http("localhost:3000").unwrap();
}
