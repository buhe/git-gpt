use std::env;
use serde::{Deserialize, Serialize};

const URL: &str = "https://api.openai.com/v1/chat/completions";

pub struct GPT {
    API_KEY: String

}

impl GPT {
    pub fn new() -> Self {
        let gpt = GPT{API_KEY: "".to_string()};
        gpt
    }
    pub fn setup(&mut self){
         match env::var("OPEN_AI") {
            Ok(val) => {
                println!("OPEN_AI is {}", val);
                self.API_KEY = val;
            },
            Err(e) => println!("couldn't read OPEN_AI: {}", e),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct GptBody {
    model: String,
    messages: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
}

async fn request_to_gpt(user_id: &str, API_KEY: String) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let gpt_body = GptBody {
        model: "gpt-3.5-turbo".to_string(),
        messages: "".to_string(), // todo log,
        temperature: Some(0.8),
        max_tokens: Some(2048),
    };

    // println!("body:--------------");
    // println!("{}", serde_json::to_string(&gpt_body).unwrap());

    let resp: serde_json::Value = reqwest::Client::new()
        .post(URL)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", API_KEY))
        .json(&gpt_body)
        .send()
        .await?
        .json()
        .await?;

    Ok(resp["choices"][0]["message"]["content"].to_string())
}