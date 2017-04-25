#[macro_use]
extern crate serde_derive;

extern crate serde_json;
extern crate iron;
extern crate hyper;

mod post;

use iron::prelude::*;
use iron::status;
use hyper::header;
use post::Post;

fn main() {

    fn hello_world(_: &mut Request) -> IronResult<Response> {
        let roots = Post::roots("/Users/shane/Desktop/zeg_bot.db");
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

    let _server = Iron::new(hello_world).http("localhost:9876");
}
