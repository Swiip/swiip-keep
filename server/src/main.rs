use reqwest::{blocking::Client, Error};
use rmp_serde::{decode, encode};
use swiip_keep_server::todo::Todo;

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

    let mut buf = Vec::new();
    encode::write(&mut buf, &todos).unwrap();
    println!("Serialized (MessagePack): {:?}", buf);

    let client = Client::new();
    let post = client
        .post("http://localhost:3000/api/handler")
        .body(buf)
        .send()?;

    println!("{:#?}", post);

    let deserialized: String = decode::from_read(post).unwrap();
    println!("Deserialized: {:?}", deserialized);

    println!("{:#?}", deserialized);

    let get = client.get("http://localhost:3000/api/handler").send()?;

    let deserialized: Vec<Todo> = decode::from_read(get).unwrap();
    println!("Deserialized: {:?}", deserialized);

    Ok(())
}
