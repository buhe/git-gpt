use std::{env, collections::VecDeque};
use serde::{Deserialize, Serialize};

const URL: &str = "https://api.openai.com/v1/chat/completions";

const promptTemplate: &str = "Write an insightful but concise Git commit message in a complete sentence in present tense for the following diff without prefacing it with anything:";

pub struct GPT {
    api_key: String

}

impl GPT {
    pub fn new() -> Self {
        let gpt = GPT{api_key: "".to_string()};
        gpt
    }
    pub fn setup(&mut self){
         match env::var("OPEN_AI") {
            Ok(val) => {
                println!("OPEN_AI is {}", val);
                self.api_key = val;
            },
            Err(e) => println!("couldn't read OPEN_AI: {}", e),
        }
    }

    pub async fn request(&self) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        self.request_to_gpt(self.api_key.clone()).await
    }

    pub async fn request_to_gpt(&self, api_key: String) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    // const prompt = `${promptTemplate}\n${diff}`;

//     const excludeFromDiff = [
// 	'package-lock.json',
// 	'pnpm-lock.yaml',

// 	// yarn.lock, Cargo.lock, Gemfile.lock, Pipfile.lock, etc.
// 	'*.lock',
// ].map(file => `:(exclude)${file}`);

// export const getStagedDiff = async () => {
// 	const diffCached = ['diff', '--cached'];
// 	const { stdout: files } = await execa(
// 		'git',
// 		[...diffCached, '--name-only', ...excludeFromDiff],
// 	);

// 	if (!files) {
// 		return;
// 	}

// 	const { stdout: diff } = await execa(
// 		'git',
// 		[...diffCached, ...excludeFromDiff],
// 	);

// 	return {
// 		files: files.split('\n'),
// 		diff,
// 	};
// };
    let msgs = VecDeque::from(vec![
                    Msg {
                        role: "system".to_string(),
                        content: "You are a helpful assistant.".to_string(),
                    },
                    Msg {
                        role: "user".to_string(),
                        content: "hey".to_string(),
                    },
                ]);
    let gpt_body = GptBody {
        model: "gpt-3.5-turbo".to_string(),
        messages: msgs, // todo log,
        temperature: Some(0.8),
        max_tokens: Some(2048),
    };

    // println!("body:--------------");
    // println!("{}", serde_json::to_string(&gpt_body).unwrap());

    let resp: serde_json::Value = reqwest::Client::new()
        .post(URL)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&gpt_body)
        .send()
        .await?
        .json()
        .await?;
    println!("raw:{}", resp);
    Ok(resp["choices"][0]["message"]["content"].to_string())
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