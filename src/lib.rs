mod client;

pub mod error;

pub mod config;

#[cfg(feature = "messages")]
pub mod messages;

pub use client::Client;
use futures::StreamExt;

use crate::messages::types::CreateMessages;

#[cfg(feature = "messages")]
#[test]
fn it_works() {
    let result = tokio::runtime::Runtime::new().unwrap().block_on(async {
        let _client = Client::new();
        let mut stream = _client.messages().create_stream(CreateMessages {}).await?;

        while let Some(event) = stream.next().await {
            println!("{:?}", event);
        }

        Ok::<(), Box<dyn std::error::Error>>(())
    });

    assert_eq!(result.is_ok(), true);
}
