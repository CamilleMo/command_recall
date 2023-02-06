use std::error::Error;

use crate::config_management::read_config_file;
use reqwest::blocking::Client;
use serde::Deserialize;
use serde_json::json;
use tabled::{Style, Table, Tabled};

pub fn make_prompt(task: String) -> String {
    let prompt = format!("Assistant is a model trained by openai.
    It is used to help linux users to find the appropriate command for a given task.
    Assistant propose 10 commands. These commands will not only show the name of the command but also an example of how to use it
    for basic usage and a basic description between parenthesis. No other text will be printed after parenthesis.
    Always use numbers from 1 to 10.
    task: {}
    commands:", task);
    prompt
}

pub fn make_request(task: String, debug: Option<bool>) -> Response {
    let client = Client::new();
    let api_key = read_config_file().token;
    let request = client
        .post("https://api.openai.com/v1/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&build_body(task));
    let response = match request.send() {
        Ok(response) => response,
        Err(err) => {
            eprintln!("Failed to send request:");
            if err.is_timeout() {
                eprintln!("Timeout, please retry later");
            }
            std::process::exit(1);
        }
    };

    match response.status().is_success() {
        true => println!("Success"),
        false => {
            eprintln!("Failure");
            eprintln!("Status: {}", response.status());
            std::process::exit(1);
        }
    }

    let response_text = response.text().unwrap();
    if debug.is_some() {
        println!("Response: {}", response_text);
    }
    // print res deserialized to Response struct
    let response: Response = serde_json::from_str(&response_text).unwrap();
    let parse_result = parse_gpt3_response(&response.choices[0]);
    let row_response = make_vec_row_from_vec_string(parse_result);
    println!("{}", Table::new(&row_response).with(Style::sharp()));
    response
}

fn build_body(task: String) -> serde_json::Value {
    let model = "text-davinci-003";
    let prompt = make_prompt(task);
    let temperature = 0.0;
    let max_tokens = 400;

    let json = json!({
        "model": model,
        "prompt": prompt,
        "temperature": temperature,
        "max_tokens": max_tokens
    });
    json
}
#[derive(Debug, Deserialize)]
pub struct Response {
    id: String,
    object: String,
    created: i64,
    model: String,
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    text: String,
    index: i64,
    finish_reason: String,
}

fn parse_gpt3_response(response: &Choice) -> Vec<String> {
    if response.finish_reason != "stop" {
        eprintln!("Error: finish_reason is not stop, please increase max_tokens");
        std::process::exit(1);
    }

    let lines = response.text.lines();
    lines.map(|line| line.to_string()).collect()
}

#[derive(Debug, Tabled)]
struct Row {
    number: String,
    command: String,
    description: String,
}

fn make_vec_row_from_vec_string(vec_string: Vec<String>) -> Vec<Row> {
    let mut vec_row = Vec::new();
    for command_row in vec_string.iter().skip(1) {
        let command_row_trimmed = command_row.trim();
        let number = command_row_trimmed.split(' ').nth(0).unwrap().to_string();

        let description = command_row_trimmed
            .split('(')
            .nth(1)
            .unwrap()
            .split(')')
            .nth(0)
            .unwrap()
            .trim().to_string();
        // extract the command
        let command_step1 = command_row_trimmed.split('(').nth(0).unwrap();
        let command = command_step1[number.len()..].trim().to_string();

        vec_row.push(Row {
            number,
            command,
            description,
        });
    }
    vec_row
}

// tests

#[test]
fn make_vec_row_from_vec_string_test() {
    let vec_string = vec![
        "".to_string(),
        "1. apt-get install (install a package)".to_string(),
        "2. apt-get remove (remove a package)".to_string(),
        "3. apt-get update (update the package list)".to_string(),
        "4. apt-get upgrade (upgrade all packages)".to_string(),
        "5. apt-get dist-upgrade (upgrade all packages)".to_string(),
        "6. apt-get autoremove (remove unused packages)".to_string(),
        "7. apt-get autoclean (remove old packages)".to_string(),
        "8. apt-get check (check for broken dependencies)".to_string(),
        "9. apt-get clean (remove old packages)".to_string(),
        "10. apt-get source (download the source code of a package)".to_string(),
    ];
    let vec_row = make_vec_row_from_vec_string(vec_string);
    assert_eq!(vec_row[0].number, "1.");
    assert_eq!(vec_row[0].command, "apt-get install");
    assert_eq!(vec_row[0].description, "install a package");
}