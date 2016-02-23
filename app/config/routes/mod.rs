use controllers::*;

pub fn routes() -> Router {
    router!(
        get "/" => posts_controller::index,
        get "/posts" => posts_controller::index,
        get "/posts/:id" => posts_controller::show,
        get "/about" => about_controller::index,
    )
}
