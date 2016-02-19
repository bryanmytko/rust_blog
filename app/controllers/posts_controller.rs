use router::Router;
use iron::{Response, Request, IronResult};
use iron::status;
use iron::mime::Mime;

use models::post;
use views::posts;

pub fn index(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((
        status::Ok,
        "text/html".parse::<Mime>().unwrap(),
        posts::index()
    )))
}

pub fn show(req: &mut Request) -> IronResult<Response> {
    let ref id = req.extensions.get::<Router>().unwrap().find("id").unwrap().parse::<i64>().unwrap();
    let post = post::Post::get_by_id(*id);

    Ok(Response::with((
        status::Ok,
        "text/html".parse::<Mime>().unwrap(),
        posts::show(post)
    )))
}
