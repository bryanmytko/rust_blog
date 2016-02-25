use controllers::*;

pub fn new(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((
        status::Ok,
        "text/html".parse::<Mime>().unwrap(),
        users::new()
    )))
}

pub fn create(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((
        status::Ok,
        "text/html".parse::<Mime>().unwrap(),
        "<h1>Not Implemented</h1>"
    )))
}
