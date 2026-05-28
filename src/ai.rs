use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};

///传给deepseek的json
#[derive(Serialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub stream: bool,
    pub thinking: EnableType,
    //pub reasoning_effort: String,
}

///系统提示词和用户输入
#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

///是否开启思考
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EnableType {
    #[serde(rename = "type")]
    pub enable_type: String,
}

impl ChatRequest {
    pub fn new() -> Self {
        ChatRequest {
            model: "deepseek-v4-flash".to_string(),
            messages: vec![Message {
                role: "system".to_string(),
                content: "你是一个ai助手".to_string(),
            }],
            stream: false,
            thinking: EnableType {
                enable_type: "disabled".to_string(),
            },
            //reasoning_effort: "high".to_string(),
        }
    }

    ///添加每次的指令
    pub fn insert_user_input(&mut self, input: &str) {
        self.messages.push(Message {
            role: "user".to_string(),
            content: input.to_string(),
        });
    }

    pub fn insert_ai_output(&mut self, input: &str) {
        self.messages.push(Message {
            role: "assistant".to_string(),
            content: input.to_string(),
        });
    }
}

#[derive(Debug, Clone)]
pub struct DeepseekRequest {
    client: Client,
    api_key: String,
    url: String,
}
impl DeepseekRequest {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            url: "https://api.deepseek.com/chat/completions".to_string(),
        }
    }

    ///调用整个的网址(这个负责调用api获取结果)
    pub async fn send_to_ds(&self, req: &mut ChatRequest) -> Result<String> {
        //TODO: 拼接链接然后发送
        let reqwest = self
            .clone()
            .client
            .post(self.clone().url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.clone().api_key))
            .json(req)
            .send()
            .await?;
        let output: serde_json::Value = reqwest.json().await?;
        println!("{:?}", output);
        Ok("test".to_string())
    }
}
