extern crate iron;
extern crate hyper;
extern crate router;

use iron::{Request, Response, IronResult};
use iron::status;
use self::hyper::mime::Mime;
use std::io::Read;
use std::fs::File;

pub fn get(_: &mut Request) -> IronResult<Response> {
    let mut f = File::open("resources/index.html").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    let content_type = "text/html".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok, s)))

}
