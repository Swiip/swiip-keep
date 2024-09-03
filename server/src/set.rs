use crate::redis::{get, REDIS_KEY, REDIS_VALUE};
use redis::Commands;
use serde_json::json;
use vercel_runtime::{Body, Error, Request, Response, StatusCode};

pub fn handler(_req: Request) -> Result<Response<Body>, Error> {
    let mut con = get()?;

    con.set(REDIS_KEY, REDIS_VALUE)?;

    let message = format!("Hello, I wrote \"{}\" in Redis!", REDIS_VALUE);

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(json!({ "message": message }).to_string().into())?)
}
