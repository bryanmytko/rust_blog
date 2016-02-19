// Supported Methods:
// get, post, put, delete, head, patch, options and any.

use router::Router;
use controllers::*;

pub fn routes() -> Router {
    router!(
        get "/" => root,
        get "/posts" => posts_controller::index,
        get "/posts/:id" => posts_controller::show
    )
}
