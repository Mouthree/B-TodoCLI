use serde::{Deserialize, Serialize};


#[derive(Serialize)]
struct ChatRequest {
    model: String,
    message: Vec<Message>,
    stream: bool
}

#[derive(Serialize, Debug, Deserialize, Clone)]
struct Message {
    role: String,
    content: String
}