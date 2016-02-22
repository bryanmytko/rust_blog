use db;

use router::Router;
use iron::{Response, Request, IronResult};
use iron::status;
use iron::mime::Mime;

use models::post;
use views::posts;

pub fn parse_id(req: &mut Request) -> i64 {
    req.extensions
        .get::<Router>()
        .unwrap()
        .find("id")
        .unwrap()
        .parse::<i64>()
        .unwrap()
}

pub fn index(_: &mut Request) -> IronResult<Response> {
    let conn = db::connection();
    let posts = post::Post::all();

    Ok(Response::with((
        status::Ok,
        "text/html".parse::<Mime>().unwrap(),
        posts::index(posts)
    )))
}

pub fn show(req: &mut Request) -> IronResult<Response> {
    let ref id = parse_id(req);
    let post = post::Post::find(*id);

    Ok(Response::with((
        status::Ok,
        "text/html".parse::<Mime>().unwrap(),
        posts::show(post)
    )))
}
