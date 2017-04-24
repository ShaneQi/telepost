#[macro_use]
extern crate serde_derive;

extern crate serde_json;
extern crate iron;
extern crate hyper;

mod post;

use iron::prelude::*;
use iron::status;
use hyper::header::{Headers, AccessControlAllowOrigin};
use post::Post;

fn main() {

    fn hello_world(_: &mut Request) -> IronResult<Response> {
        let roots = Post::roots("/Users/shane/Desktop/zeg_bot.db");
        let json = serde_json::to_string(&roots).unwrap();
        let mut resp = Response::with((status::Ok, json));

        let mut headers = Headers::new();
        headers.set(
            AccessControlAllowOrigin::Any
        );

        resp.headers = headers;
        Ok(resp)
    }

    let _server = Iron::new(hello_world).http("localhost:9876").unwrap();
}
