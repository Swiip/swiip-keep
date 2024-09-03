use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Todo {
    pub id: u32,
    pub text: String,
    pub completed: bool,
}
