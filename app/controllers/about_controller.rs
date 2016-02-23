use controllers::*;

pub fn index(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((
        status::Ok,
        "text/html".parse::<Mime>().unwrap(),
        about::index()
    )))
}
