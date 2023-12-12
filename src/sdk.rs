use std::{env, collections::VecDeque};
use serde::{Deserialize, Serialize};

const URL: &str = "https://api.openai.com";

const PROMPT_TEMPLATE: &str = "Write an insightful but concise Git commit message in a complete sentence in present tense for the following diff without prefacing it with anything:";
const PROMPT_TEMPLATE2: &str = "The generated message must not exceed 200 words, Word count is important ";


pub struct GPT {
    api_key: String,
    proxy: Option<String>
    // model: Option<String>
}

impl GPT {
    pub fn new() -> Self {
        let gpt = GPT{api_key: "".to_string(), proxy: None};
        gpt
    }
    pub fn setup(&mut self) -> bool {
         match env::var("OPENAI_API_KEY") {
            Ok(val) => {
                // println!("OPEN_AI is {}", val);
                self.api_key = val;

                 match env::var("OPENAI_URL") {
                    Ok(url) => {
                        self.proxy = Some(url)
                    },
                    Err(_) => { 
                    },
                }

                return true;

            },
            Err(e) => { 
                println!("couldn't read OPENAI_API_KEY: {}", e);
                return false;
            },
        }
    }

    pub async fn request(&self, diff: String, verbose: bool) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let msg = format!("{} {} {}",PROMPT_TEMPLATE, diff, PROMPT_TEMPLATE2);
        self.request_to_gpt(self.api_key.clone(), msg, verbose).await
    }

    pub async fn request_to_gpt(&self, api_key: String, msg: String, verbose: bool) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        
    let msgs = VecDeque::from(vec![
                    Msg {
                        role: "system".to_string(),
                        content: "You are a helpful assistant.".to_string(),
                    },
                    Msg {
                        role: "user".to_string(),
                        content: msg,
                    },
                ]);
    let gpt_body = GptBody {
        model: "gpt-3.5-turbo-16k".to_string(),
        messages: msgs,
        temperature: Some(0.0),     
        max_tokens: Some(15000),
    };

    let url: &str = if self.proxy.is_none() { URL } else { self.proxy.as_ref().unwrap().as_str() };
    let resp = reqwest::Client::new()
        .post([url , "/v1/chat/completions"].join(""))
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&gpt_body)
        .send()
        .await?;
        
    if verbose {
        println!("gpt raw:{:?}", resp);
    }
    let json: serde_json::Value = resp.json()
        .await?;
    Ok(json["choices"][0]["message"]["content"].to_string())
}
}

#[derive(Serialize, Deserialize, Debug)]
struct GptBody {
    model: String,
    messages: VecDeque<Msg>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
}



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Msg {
    role: String,
    content: String,
}