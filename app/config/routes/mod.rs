// Supported Methods:
// get, post, put, delete, head, patch, options and any.

use router::Router;
use controllers::*;

pub fn routes() -> Router {
    // get "/" => root,

    router!(
        get "/" => posts_controller::index,
        get "/posts" => posts_controller::index,
        get "/posts/:id" => posts_controller::show
    )
}
