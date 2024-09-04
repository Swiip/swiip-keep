use crate::{
    redis::{get, REDIS_KEY},
    todo::Todo,
};
use redis::Commands;
use rmp_serde::encode;
use vercel_runtime::{Body, Error, Request, Response, StatusCode};

pub fn handler(_req: Request) -> Result<Response<Body>, Error> {
    let mut con = get()?;

    let read: String = con.get(REDIS_KEY)?;

    println!("I read \"{}\" in Redis!", &read);

    let received: Vec<Todo> = serde_json::from_str(&read)?;

    let mut buf = Vec::new();
    encode::write(&mut buf, &received).unwrap();
    println!("Serialized (MessagePack): {:?}", buf);

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/x-msgpack")
        .body(buf.into())?)
}
