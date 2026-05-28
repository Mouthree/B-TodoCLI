use reqwest::Client;
use serde::{Deserialize, Serialize};

///传给deepseek的json
#[derive(Serialize)]
pub struct ChatRequest {
    pub model: String,
    pub message: Vec<Message>,
    pub stream: bool,
    pub thinking: EnableType,
    pub reasoning_effort: String,
}

///系统提示词和用户输入
#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: String
}

///是否开启思考
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EnableType {
    #[serde(rename = "type")]
    pub enable_type: String
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

    ///添加每次的指令
    pub fn insert_user_input(&mut self, input: &str) {
        self.message.push(Message { role: "user".to_string(), content: input.to_string() });
    }
}


#[derive(Debug, Clone)]
pub struct DeepseekRequest {
    http: Client,
    api_key: String,
    url: String
}
impl DeepseekRequest {
    pub fn new(api_key: String) -> Self {
        Self { http: Client::new(), api_key, url: "".to_string() }
    }
}