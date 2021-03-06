#[macro_use]
extern crate serde_derive;

extern crate serde_json;
extern crate iron;
extern crate hyper;

mod post;
mod secret;

use iron::prelude::*;
use iron::status;
use hyper::header;
use post::Post;

fn main() {

    fn hello_world(_: &mut Request) -> IronResult<Response> {
        let roots = match Post::roots(&secret::db_path()) {
            Ok(roots) => roots,
            Err(_) => return Ok(Response::with(status::InternalServerError)),
        };
        let json = serde_json::to_string(&roots);
        match json {
            Ok(json_string) => {
                let mut resp = Response::with((status::Ok, json_string));
                let mut headers = header::Headers::new();
                headers.set(header::AccessControlAllowOrigin::Any);
                headers.set(header::ContentType::json());
                resp.headers = headers;
                Ok(resp)
            }
            Err(_) => Ok(Response::with(status::InternalServerError)),
        }
    }

    let _server = Iron::new(hello_world).http("0.0.0.0:9876");
}
