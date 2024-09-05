use reqwest::Client;
use rmp_serde::{decode, encode};
use serde::{Deserialize, Serialize};
use std::error::Error;


// const URL: &str = "http://localhost:3000/api/handler";
const URL: &str = "https://server-psi-ochre.vercel.app/api/handler";

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Todo {
    pub id: String,
    pub text: String,
    pub completed: bool,
}

pub async fn load_todos() -> Result<Vec<Todo>, Box<dyn Error>> {
    let client = Client::new();

    let get_response = client.get(URL).send().await?;
    println!("get response: {:#?}", get_response);

    let get_bytes = get_response.bytes().await?;

    let todos: Vec<Todo> = decode::from_slice(&get_bytes)?;
    println!("todos: {:#?}", todos);

    Ok(todos)
}

pub async fn save_todos(todos: &Vec<Todo>) -> Result<String, Box<dyn Error>> {
    let mut buf = Vec::new();
    encode::write(&mut buf, todos)?;

    let client = Client::new();

    let post_response = client.post(URL).body(buf).send().await?;
    println!("post response: {:#?}", post_response);

    let post_bytes = post_response.bytes().await?;

    let post_response_payload: String = decode::from_slice(&post_bytes)?;
    println!("post response payload: {:#?}", post_response_payload);

    Ok(post_response_payload)
}
