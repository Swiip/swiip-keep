use std::io::Cursor;

use crate::todo::Todo;
use rmp_serde::{decode, encode};
use serde::Serialize;
use vercel_runtime::{Body, Error, Request, Response, StatusCode};

pub fn encode(payload: &impl Serialize) -> Result<Vec<u8>, Error> {
    let mut buf = Vec::new();
    encode::write(&mut buf, payload)?;

    Ok(buf)
}

pub fn build_http_response(payload: &impl Serialize) -> Result<Response<Body>, Error> {
    let buf = encode(payload)?;
    println!("http response: {:?}", buf);

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/x-msgpack")
        .body(buf.into())?)
}

pub fn read_http_body(req: Request) -> Result<Vec<Todo>, Error> {
    let cursor = Cursor::new(req.body());
    let todos: Vec<Todo> = decode::from_read(cursor).unwrap();
    println!("http body: {:#?}", todos);

    Ok(todos)
}
