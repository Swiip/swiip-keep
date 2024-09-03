use std::io::Cursor;

use rmp_serde::{decode, encode};
use swiip_keep_server::todo::Todo;

fn main() {
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

    let cursor = Cursor::new(buf);
    let deserialized: Vec<Todo> = decode::from_read(cursor).unwrap();
    println!("Deserialized: {:?}", deserialized);
}
