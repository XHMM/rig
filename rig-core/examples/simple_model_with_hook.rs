use std::sync::Arc;

use rig::{completion::Prompt, providers::openai};

#[tokio::main]
async fn main() {
    // Create OpenAI client and model
    let openai_client = openai::Client::from_env()
        .with_pre_request_hook(Arc::new(|request| {
            println!(
                "Request: {}",
                serde_json::to_string_pretty(&request).unwrap()
            );
        }))
        .with_post_response_hook(Arc::new(|response| {
            println!("Response: {:?}", response);
        }));

    let gpt4 = openai_client.agent("gpt-4").build();

    // Prompt the model and print its response
    let response = gpt4
        .prompt("Who are you?")
        .await
        .expect("Failed to prompt GPT-4");

    println!("GPT-4: {response}");
}
