use redis::{Client, Commands};
use regex::Regex;
use serde_json::json;
use std::env;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

const REDIS_KEY: &str = "my_key";
// TO CHANGE
const REDIS_VALUE: &str = "this is the way!";

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    let kv_url = env::var("KV_URL")?;

    println!("kv_url: {}", kv_url);

    let url = Regex::new("^redis:")
        .unwrap()
        .replace_all(&kv_url, "rediss:")
        .to_string();

    println!("url: {}", url);

    let client = Client::open(url)?;

    let mut con = client.get_connection()?;

    con.set(REDIS_KEY, REDIS_VALUE)?;

    let message = format!("Hello, I wrote \"{}\" in Redis!", REDIS_VALUE);

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(json!({ "message": message }).to_string().into())?)
}
