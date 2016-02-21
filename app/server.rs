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
use rusqlite::Connection;

pub mod config;
pub mod models;
pub mod views;
pub mod controllers;

fn main() {
    let conn = Connection::open_in_memory().unwrap();

    conn.execute(
    "CREATE TABLE person (
        id              INTEGER PRIMARY KEY,
        name            TEXT NOT NULL,
        time_created    TEXT NOT NULL,
        data
        BLOB
    )", &[]).unwrap();











    let mut mount = Mount::new();

    mount
        .mount("/", config::routes::routes())
        .mount("/assets", Static::new(Path::new("public/assets/")));

    Iron::new(mount).http("localhost:3000").unwrap();
}
