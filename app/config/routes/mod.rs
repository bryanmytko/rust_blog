// Supported Methods:
// get, post, put, delete, head, patch, options and any.

use router::Router;
use iron::{Response, Request, IronResult};
use iron::status;
use controllers::*;

pub fn routes() -> Router {

    router!(
        get "/" => root,
        get "/:resource/:verb/:id" => query_handler
        // post "/" => postHandler
    )
}
