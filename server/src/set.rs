use crate::{
    redis::{get_redis_connection, write_todos},
    serialization::{build_http_response, read_http_body},
};
use vercel_runtime::{Body, Error, Request, Response};

pub fn handler(req: Request) -> Result<Response<Body>, Error> {
    let todos = read_http_body(req)?;

    let mut con = get_redis_connection()?;

    write_todos(&mut con, todos)?;

    build_http_response(&"ok")
}
