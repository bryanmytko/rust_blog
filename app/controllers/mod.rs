pub use iron::{Response, Request, IronResult};
pub use iron::status;
pub use router::Router;
pub use iron::mime::Mime;

pub use views::*;
pub use models::post::*;

pub mod posts_controller;
pub mod about_controller;

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
