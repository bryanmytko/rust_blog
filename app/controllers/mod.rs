use iron::{Response, Request, IronResult};
use iron::status;
use router::Router;

use iron::mime::Mime;

pub mod posts_controller;

pub fn root(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((
        status::Ok,
        html_mime_type(),
        format!("<h1>Home Page</h1>")
    )))
}

pub fn get_handler(req: &mut Request) -> IronResult<Response> {
    let ref query = req
        .extensions
        .get::<Router>()
        .unwrap()
        .find("query")
        .unwrap();

        Ok(Response::with((
            status::Ok,
            html_mime_type(),
            format!("<h1>Home Page {}</h1>", query)
        )))


    // match query {
    //     _ => posts::index
    // }
    //
    // return req;


}


pub fn html_mime_type() -> Mime {
    "text/html".parse::<Mime>().unwrap()
}


