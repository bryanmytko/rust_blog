#![feature(plugin)]
#![plugin(maud_macros)]

extern crate iron;
extern crate rustc_serialize;
extern crate maud;

#[macro_use]
extern crate router;

use iron::prelude::*;
use std::io;

pub mod config;
pub mod models;
pub mod views;
pub mod controllers;

fn main() {
    //Iron::new(config::routes::routes()).http("localhost:3000").unwrap();

    let name = "Lyra";
    html_utf8!(io::stdout(), {
        p { "Hi, " $name "!" }
    }).unwrap();
}
