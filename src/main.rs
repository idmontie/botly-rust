extern crate iron;
extern crate rustc_serialize;

mod http;
mod routes;

use iron::prelude::*;
use iron::status;

fn main() {
    Iron::new(routes::routes()).http("localhost:3000").unwrap();
}
