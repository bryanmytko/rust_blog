use controllers::*;

pub fn new(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((
        status::Ok,
        "text/html".parse::<Mime>().unwrap(),
        users::new()
    )))
}

pub fn create(_: &mut Request) -> IronResult<Response> {
    let params: [String; 5] = [
        "a@b.com".to_string(),
        "foo".to_string(),
        "bar".to_string(),
        "asdf".to_string(),
        "false".to_string(),
    ];

    let user = User::create(&params);

    Ok(Response::with((
        status::Ok,
        "text/html".parse::<Mime>().unwrap(),
        "<h1>Ok</h1>"
    )))
}
