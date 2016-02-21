use iron::{Response, Request, IronResult};
use iron::status;

use iron::mime::Mime;

pub mod posts_controller;
use views::layouts;

pub fn root(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((
        status::Ok,
        html_mime_type(),
        layouts::default("<h1>Homepage</h1><a href=\"/posts\">See Posts &#xbb;</a>".to_string())
    )))
}

pub fn html_mime_type() -> Mime {
    "text/html".parse::<Mime>().unwrap()
}
