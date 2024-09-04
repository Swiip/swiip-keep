use crate::{
    redis::{get_redis_connection, read_todos},
    serialization::build_http_response,
};
use vercel_runtime::{Body, Error, Request, Response};

pub fn handler(_req: Request) -> Result<Response<Body>, Error> {
    let mut con = get_redis_connection()?;

    let todos = read_todos(&mut con)?;

    build_http_response(&todos)
}
