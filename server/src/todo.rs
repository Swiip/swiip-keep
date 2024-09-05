use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Todo {
    pub id: String,
    pub text: String,
    pub completed: bool,
}
