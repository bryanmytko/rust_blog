use router::Router;
use iron::{Response, Request, IronResult};
use iron::status;
use iron::mime::Mime;

pub fn index(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((
        status::Ok,
        "text/html".parse::<Mime>().unwrap(),
        format!("<h1>Index Page</h1>")
    )))
}

pub fn show(req: &mut Request) -> IronResult<Response> {
    let ref id = req.extensions.get::<Router>().unwrap().find("id").unwrap().parse::<i64>().unwrap();

    Ok(Response::with((
        status::Ok,
        "text/html".parse::<Mime>().unwrap(),
        format!("<h1>Show Page ID: {}</h1>", id)
    )))
}
