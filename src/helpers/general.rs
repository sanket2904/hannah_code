use std::fs;

use reqwest::Client;
use serde::de::DeserializeOwned;
use crate::{models::general::llm::Message, apis::call_request::call_gpt};
use super::command_line::PrintCommand;


const CODE_TEMPLATE_PATH: &str = "/home/ssa006/data_sync/src/gpt_created.rs";

pub fn extend_ai_functions(ai_func: fn(&str) -> &'static str, func_input: &str) -> Message {
    let ai_func_output = ai_func(func_input);
    let msg = format!("FUNCTION {} 
    INSTRUCTION: You are a function printer. You ONLY print the results of functions.
    Nothing else. No commentary. Here is the input to the function {}.
    Print out what the function will return.", ai_func_output, func_input);
    Message { role: "system".to_string() , content: msg }
}
pub async fn ai_task_request(msg_context: String, agent_position: &str, agent_operation: &str, function_pass: fn(&str) -> &'static str) -> String {
    let func_message = extend_ai_functions(function_pass, &msg_context);
    PrintCommand::AICall.print_agent_message(agent_position, agent_operation);
    let llm_response_res = call_gpt(vec![func_message.clone()]).await;
    match llm_response_res {
        Ok(r) => r,
        Err(_) => {
            call_gpt(vec![func_message]).await.expect("Failed to call GPT4")
        }
    }.choices[0].message.content.clone()
}
pub async fn ai_task_request_decoded<T: DeserializeOwned>(msg_context: String, agent_position: &str, agent_operation: &str, function_pass: fn(&str) -> &'static str) -> T {
    let llm_res = ai_task_request(msg_context, agent_position, agent_operation, function_pass).await;
    let llm_res_decoded: T = serde_json::from_str(&llm_res).expect("Failed to decode LLM response");
    llm_res_decoded
}
// check if req url is valid
pub async fn check_status_code(url: &str) -> Result<u16, reqwest::Error> {
    let client = Client::builder().timeout(std::time::Duration::from_secs(5)).build()?;
    let res = client.get(url).send().await;

    // let res = reqwest::get(url).await  ;
    match res {
        Ok(r) => Ok(r.status().as_u16()),
        Err(e) => Err(e)
    }
}
// Get code template
pub fn read_code_template() -> String {
    fs::read_to_string(String::from(CODE_TEMPLATE_PATH)).expect("Failed to read code template")
}

pub fn read_exec_main_contents() -> String {
    fs::read_to_string(String::from("/home/ssa006/data_sync/src/main.rs")).expect("Failed to read exec main contents")
}

// Save new backend code
pub fn save_backend_code(contents: &str) {
    fs::write(String::from("/home/ssa006/data_sync/src/main.rs"), contents).expect("Failed to write new backend code");
}
// Save Json api Endpoint Schema
pub fn save_api_endpoints(contents: &str) {
    fs::write(String::from("/home/ssa006/data_sync/src/api_endpoints.json"), contents).expect("Failed to write new api endpoints");
}
#[cfg(test)]
mod test {
    use super::{extend_ai_functions, ai_task_request};
    use crate::ai_functions::aifunc_managing::convert_user_input_to_goal;

    #[tokio::test]
    async fn test_extend_ai_functions() {
        let ai_fun = "Build me a web server which can receive the ios front facing camera feed.".to_string();
        let res = ai_task_request(ai_fun, "Managing Agent","Defining user requirements" , convert_user_input_to_goal).await;
        dbg!(res);
    }
}