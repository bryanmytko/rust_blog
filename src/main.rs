extern crate iron;

#[macro_use]
extern crate router;

use iron::prelude::*;
use iron::status;

use router::Router;
mod config;

fn main() {
    Iron::new(config::router::router).http("localhost:3000").unwrap();

    fn handler(req: &mut Request) -> IronResult<Response> {
        let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap();
        Ok(Response::with((status::Ok, format!("Hello Universe. You came here via /{}", query))))
    }
}


