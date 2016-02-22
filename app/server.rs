#![feature(plugin)]
#![plugin(maud_macros)]

extern crate iron;
extern crate rustc_serialize;
extern crate staticfile;
extern crate mount;
extern crate maud;
extern crate rusqlite;

#[macro_use]
extern crate router;

use iron::prelude::*;
use staticfile::Static;
use mount::Mount;
use std::path::Path;

pub mod config;
pub mod models;
pub mod views;
pub mod controllers;
pub mod db;

const DB_NAME: &'static str = "./blog.sql";

fn main() {
    // let sqlite_path = Path::new("./blog.sql");
    // let conn = Connection::open(sqlite_path).unwrap();

    let mut mount = Mount::new();

    mount
        .mount("/", config::routes::routes())
        .mount("/assets", Static::new(Path::new("public/assets/")));

    Iron::new(mount).http("localhost:3000").unwrap();
}
