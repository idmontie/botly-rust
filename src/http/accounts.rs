extern crate iron;
extern crate router;

use iron::{Request, Response, IronResult};
use iron::status;

pub fn get(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "GET")))
}

pub fn post(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "POST")))
}

pub fn put(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "PUT")))
}

pub fn delete(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "DELETE")))
}

pub fn options(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "OPTIONS")))
}
