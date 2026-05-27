use serde::{Deserialize, Serialize};


#[derive(Serialize)]
struct ChatRequest {
    model: String,
    message: Vec<Message>,
    stream: bool,
    thinking: EnableType,
    reasoning_effort: String,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
struct Message {
    role: String,
    content: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct EnableType {
    type: String
}