use futures::StreamExt;
use openai_dive::v1::api::Client;
use openai_dive::v1::resources::chat::{ChatCompletionParameters, ChatMessage, Role};
use std::env;

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = ChatCompletionParameters {
        model: "gpt-3.5-turbo-16k-0613".to_string(),
        messages: vec![
            ChatMessage {
                role: Role::User,
                content: Some("Hello!".to_string()),
                ..Default::default()
            },
            ChatMessage {
                role: Role::User,
                content: Some("Where are you located?".to_string()),
                ..Default::default()
            },
        ],
        max_tokens: Some(12),
        ..Default::default()
    };

    let mut stream = client.chat().create_stream(parameters).await.unwrap();

    while let Some(response) = stream.next().await {
        match response {
            Ok(chat_response) => chat_response.choices.iter().for_each(|choice| {
                if let Some(content) = &choice.delta.content {
                    print!("{}", content);
                }
            }),
            Err(e) => eprintln!("{}", e),
        }
    }
}
