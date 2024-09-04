use crate::{
    redis::{get, REDIS_KEY},
    todo::Todo,
};
use redis::Commands;
use rmp_serde::{decode, encode};
use serde_json::json;
use std::io::Cursor;
use vercel_runtime::{Body, Error, Request, Response, StatusCode};

pub fn handler(_req: Request) -> Result<Response<Body>, Error> {
    let cursor = Cursor::new(_req.body());
    let deserialized: Vec<Todo> = decode::from_read(cursor).unwrap();
    println!("Deserialized: {:?}", deserialized);

    let received = json!(deserialized).to_string();

    let mut con = get()?;

    con.set(REDIS_KEY, &received)?;

    println!("I wrote \"{}\" in Redis!", &received);

    let mut buf = Vec::new();
    encode::write(&mut buf, "ok").unwrap();
    println!("Serialized (MessagePack): {:?}", buf);

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/x-msgpack")
        .body(buf.into())?)
}
