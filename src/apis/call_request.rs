use crate::models::general::llm::{Message, ChatCompletion,APIResponse};
use reqwest::{Client, header::HeaderValue};
use std::{env, fmt::Error};

pub async fn call_gpt(messages: Vec<Message>) -> Result<APIResponse, Box<dyn std::error::Error + Send>> {
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not found");
    let org_id = env::var("ORG_ID").expect("OPENAI_ORG_ID not found");
    // establish the headers
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("authorization",  HeaderValue::from_str(&format!("Bearer {}", api_key)).map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) } )?);
    headers.insert("OpenAI-Organization",  HeaderValue::from_str(org_id.as_str()).map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) } )?);
    let client = Client::builder().default_headers(headers).build().map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) } )?;
    let chat_completion = ChatCompletion{
        model: "gpt-4".to_string(),
        messages: messages,
        temperature: 0.1,
    };
    let res = client.post("https://api.openai.com/v1/chat/completions").json(&chat_completion).send().await.map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) } )?;
    let res = res.json::<APIResponse>().await.map_err(|e| ->  Box<dyn std::error::Error + Send> { Box::new(e) } )?;
    Ok(res)
}

// crete a test 

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_call_gpt() {
        let messages = vec![Message{
            role: "user".to_string(),
            content: "How to go to deep trace yourself without help of a hypnotist.".to_string(),
        }];
        call_gpt(messages).await;
    }
}