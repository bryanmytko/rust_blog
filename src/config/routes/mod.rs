// Supported Methods:
// get, post, put, delete, head, patch, options and any.

use router::Router;
use iron::{Response, Request, IronResult};
use iron::status;

pub fn routes() -> Router {
    fn index_handler(_: &mut Request) -> IronResult<Response> {
        Ok(Response::new())
    }

    fn query_handler(req: &mut Request) -> IronResult<Response> {
        let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap();
        Ok(Response::with((status::Ok, format!("Hello Universe. You came here via {}", query))))
    }

    router!(
        get "/" => index_handler,
        get "/:query" => query_handler
        // post "/" => postHandler
    )
}
