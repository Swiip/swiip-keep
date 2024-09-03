use redis::{Client, Connection};
use regex::Regex;
use std::env;
use vercel_runtime::Error;

pub const REDIS_KEY: &str = "my_key";
// TO CHANGE
pub const REDIS_VALUE: &str = "this is the way!";

pub fn get() -> Result<Connection, Error> {
    let kv_url = env::var("KV_URL")?;

    println!("kv_url: {}", kv_url);

    let url = Regex::new("^redis:")
        .unwrap()
        .replace_all(&kv_url, "rediss:")
        .to_string();

    println!("url: {}", url);

    let client = Client::open(url)?;

    let con = client.get_connection()?;

    Ok(con)
}
