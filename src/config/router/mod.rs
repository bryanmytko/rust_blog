// Supported Methods: get, post, put, delete, head, patch, options and any.

pub fn router() -> Router {
    let router = router!(
        get "/" => "",
        // get "/:id" => queryHandler,
        // post "/" => postHandler
    );

    return router;
}
