use reqwest::{blocking::Client, Error};
use rmp_serde::encode;
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
            text: String::from("second task"),
            completed: false,
        },
    ];

    let mut buf = Vec::new();
    encode::write(&mut buf, &todos).unwrap();
    println!("Serialized (MessagePack): {:?}", buf);

    let client = Client::new();
    let res = client
        .post("http://localhost:3000/api/handler")
        .body(buf)
        .send()?;

    println!("{:#?}", res);

    let content = res.text();

    println!("{:#?}", content);

    // let cursor = Cursor::new(buf);
    // let deserialized: Vec<Todo> = decode::from_read(cursor).unwrap();
    // println!("Deserialized: {:?}", deserialized);

    Ok(())
}
