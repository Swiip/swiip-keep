use reqwest::blocking::Client;
use rmp_serde::decode;
use swiip_keep_server::{serialization::encode, todo::Todo};
use vercel_runtime::Error;

fn main() -> Result<(), Error> {
    let todos = vec![
        Todo {
            id: 1,
            text: String::from("very first task"),
            completed: true,
        },
        Todo {
            id: 2,
            text: String::from("second task changed"),
            completed: false,
        },
    ];

    let client = Client::new();

    let post_response = client
        .post("http://localhost:3000/api/handler")
        .body(encode(&todos)?)
        .send()?;

    println!("post response: {:#?}", post_response);

    let post_response_payload: String = decode::from_read(post_response)?;
    println!("post response payload: {:#?}", post_response_payload);

    let get_response = client.get("http://localhost:3000/api/handler").send()?;
    println!("get response: {:#?}", get_response);

    let todos: Vec<Todo> = decode::from_read(get_response)?;
    println!("todos: {:#?}", todos);

    Ok(())
}
