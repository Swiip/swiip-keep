use reqwest::blocking::Client;
use rmp_serde::{decode, encode};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Todo {
    pub id: String,
    pub text: String,
    pub completed: bool,
}

pub fn load_todos() -> Result<Vec<Todo>, Box<dyn Error>> {
    let client = Client::new();

    let get_response = client.get("http://localhost:3000/api/handler").send()?;
    println!("get response: {:#?}", get_response);

    let todos: Vec<Todo> = decode::from_read(get_response)?;
    println!("todos: {:#?}", todos);

    Ok(todos)
}

pub fn save_todos(todos: &Vec<Todo>) -> Result<String, Box<dyn Error>> {
    let mut buf = Vec::new();
    encode::write(&mut buf, todos)?;

    let client = Client::new();

    let post_response = client
        .post("http://localhost:3000/api/handler")
        .body(buf)
        .send()?;

    println!("post response: {:#?}", post_response);

    let post_response_payload: String = decode::from_read(post_response)?;
    println!("post response payload: {:#?}", post_response_payload);

    Ok(post_response_payload)
}
