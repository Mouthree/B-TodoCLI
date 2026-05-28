use serde::{Deserialize, Serialize};

///传给deepseek的json
#[derive(Serialize)]
struct ChatRequest {
    model: String,
    message: Vec<Message>,
    stream: bool,
    thinking: EnableType,
    reasoning_effort: String,
}

///系统提示词和用户输入
#[derive(Serialize, Debug, Deserialize, Clone)]
struct Message {
    role: String,
    content: String
}

///是否开启思考
#[derive(Serialize, Deserialize, Debug, Clone)]
struct EnableType {
    enable_type: String
}

impl ChatRequest {
    pub fn new() -> Self {
        ChatRequest { 
            model: "deepseek-v4-lite".to_string(),
            message: vec![Message { role: "system".to_string(), content: "提示词".to_string()}],
            stream: false,
            thinking: EnableType { enable_type: "disable".to_string() },
            reasoning_effort: "high".to_string()
        }
    }
}