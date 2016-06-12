extern crate iron;
extern crate router;

use iron::{Request, Response, IronResult};
use iron::status;

pub fn get(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Home")))
}
