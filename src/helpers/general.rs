use crate::apis::call_request::call_gpt;
use crate::helpers::command_lines::PrintCommand;
use crate::models::general::llm::Message;
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::fs;

const CODE_TEMPLATE_PATH: &str = "/Users/Ramon/githubprojects/web_template/src/code_template.rs";
pub const WEB_SERVER_PROJECT_PATH: &str = "/Users/Ramon/githubprojects/web_template/";
const EXEC_MAIN_PATH: &str = "/Users/Ramon/githubprojects/web_template/src/main.rs";
const API_SCHEMA_PATH: &str = "/Users/Ramon/auto_gippity/schemas/api_schema.json";

//Extend ai function to encourage certain specific output
pub fn extend_ai_function(ai_func: fn(&str) -> &'static str, func_input: &str) -> Message {
    let ai_function_str = ai_func(func_input);

    //Extend the string to encourage only printing the output
    let msg = format!(
        "FUNCTION {ai_function_str}\
    INSTRUCTIONS: You are a function printer. You ONLY print the result of functions.\

    Nothing else. No commentary. Here is the input to the function {func_input}.\
    Print out what the functions will return"
    );

    //Return message
    Message {
        role: "system".to_string(),
        content: msg,
    }
}
// Performs call to LLM GPT
pub async fn ai_task_request(
    msg_context: String,
    agent_position: &str,
    agent_operation: &str,
    function_pass: for<'a> fn(&'a str) -> &'static str,
) -> String {
    //Extend AI function
    let extended_msg = extend_ai_function(function_pass, &msg_context);
    //Print current State
    PrintCommand::AICall.print_agent_message(agent_position, agent_operation);

    //Get LLM response
    let llm_response_res = call_gpt(vec![extended_msg.clone()]).await;

    //Return Success or try again
    let llm_response = match llm_response_res {
        Ok(llm_resp) => llm_resp,

        Err(_) => call_gpt(vec![extended_msg.clone()])
            .await
            .expect("Failed twice to call OpenAI"),
    };
    String::from("something")
}

// Performs call to LLM GPT - Decoded
pub async fn ai_task_request_decoded<T: DeserializeOwned>(
    msg_context: String,
    agent_position: &str,
    agent_operation: &str,
    function_pass: for<'a> fn(&'a str) -> &'static str,
) -> T {
    let llm_response =
        ai_task_request(msg_context, agent_position, agent_operation, function_pass).await;

    let decoded_response: T = serde_json::from_str(llm_response.as_str())
        .expect("Fail to decode ai response from serde_json");

    decoded_response
}

//Check whether request url is valid
pub async fn check_status_code(client: &Client, url: &str) -> Result<u16, reqwest::Error> {
    let response = client.get(url).send().await?;
    Ok(response.status().as_u16())
}

// Get Code Template
pub fn read_code_template_contents() -> String {
    let path: String = String::from(CODE_TEMPLATE_PATH);
    fs::read_to_string(path).expect("Failed to read code template")
}
//GET Exec Main
pub fn read_exec_main_contents() -> String{
    let path = String::from(EXEC_MAIN_PATH);
    fs::read_to_string(path).expect("Failed to read template")
}
// Save new Backend Code
pub fn save_backend_code(contents: &String) {
    let path: String = String::from(EXEC_MAIN_PATH);
    fs::write(path, contents).expect("Failed to write main.rs file")
}
//Save the JSON API Endpoint Schema
pub fn save_api_endpoints(api_endpoints: &String) {
    let path: String = String::from(API_SCHEMA_PATH);
    fs::write(path, api_endpoints).expect("Failed to write API Endpoints to file")
}
#[cfg(test)]

mod tests {
    use super::*;
    use crate::ai_functions::aifunc_managing::convert_user_input_to_goal;
    #[test]
    fn tests_extending_ai_function() {
        let extended_msg = extend_ai_function(convert_user_input_to_goal, "dummy variable");
        dbg!(&extended_msg);
        assert_eq!(extended_msg.role, "system".to_string());
    }

    #[tokio::test]
    async fn tests_ai_task_request() {
        let ai_func_param = "Build me a webserver for making stock price api requests.".to_string();
        let res = ai_task_request(
            ai_func_param,
            "Managing Agent",
            "Defining use requirements",
            convert_user_input_to_goal,
        )
        .await;

        assert!(res.len() > 20);
    }
}
