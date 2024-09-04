use redis::{Client, Commands, Connection};
use regex::Regex;
use serde_json::json;
use std::env;
use vercel_runtime::Error;

use crate::todo::Todo;

pub const REDIS_KEY: &str = "my_key";

pub fn get_redis_connection() -> Result<Connection, Error> {
    let kv_url = env::var("KV_URL")?;

    println!("kv_url: {:#?}", kv_url);

    let url = Regex::new("^redis:")
        .unwrap()
        .replace_all(&kv_url, "rediss:")
        .to_string();

    println!("url: {:#?}", url);

    let client = Client::open(url)?;

    let con = client.get_connection()?;

    Ok(con)
}

pub fn read_todos(con: &mut Connection) -> Result<Vec<Todo>, Error> {
    let read: String = con.get(REDIS_KEY)?;

    println!("redis read: {:#?}", &read);

    let todos: Vec<Todo> = serde_json::from_str(&read)?;

    Ok(todos)
}

pub fn write_todos(con: &mut Connection, todos: Vec<Todo>) -> Result<(), Error> {
    let serialized = json!(todos).to_string();

    let write: String = con.set(REDIS_KEY, &serialized)?;

    println!("redis write: {:#?}", &write);

    Ok(())
}
