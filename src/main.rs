extern crate iron;

#[macro_use]
extern crate router;

use iron::prelude::*;

mod config;

fn main() {
    Iron::new(config::routes::routes()).http("localhost:3000").unwrap();
}


