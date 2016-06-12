extern crate iron;
extern crate router;

use self::router::Router;
use http;

pub fn routes() -> Router {
    let mut router = Router::new();

    router.get("/", http::home::get);

    router.get("/accounts", http::accounts::get);
    router.post("/accounts", http::accounts::post);
    router.put("/accounts", http::accounts::put);
    router.delete("/accounts", http::accounts::delete);
    router.options("/accounts", http::accounts::options);

    return router;
}
