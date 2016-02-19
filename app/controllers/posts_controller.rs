use router::Router;
use iron::{Response, Request, IronResult};
use iron::status;
use iron::mime::Mime;

use models::post;

pub fn index(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((
        status::Ok,
        "text/html".parse::<Mime>().unwrap(),
        format!("<h1>Index Page</h1>")
    )))
}

pub fn show(req: &mut Request) -> IronResult<Response> {
    let ref id = req.extensions.get::<Router>().unwrap().find("id").unwrap().parse::<i64>().unwrap();
    let post = post::Post::get_by_id(*id);

    Ok(Response::with((
        status::Ok,
        "text/html".parse::<Mime>().unwrap(),
        format!(
            "<div><h1>{}</h1><em>by: {}</em><p>{}</p></div>",
            post.title,
            post.author,
            post.content
        )
    )))
}
