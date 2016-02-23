use controllers::*;

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
    let posts = Post::all();

    Ok(Response::with((
        status::Ok,
        "text/html".parse::<Mime>().unwrap(),
        posts::index(posts)
    )))
}

pub fn show(req: &mut Request) -> IronResult<Response> {
    let ref id = parse_id(req);
    let post = Post::find(*id);

    Ok(Response::with((
        status::Ok,
        "text/html".parse::<Mime>().unwrap(),
        posts::show(post)
    )))
}
