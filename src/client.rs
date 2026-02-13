use futures::future;
use futures::{StreamExt, stream::BoxStream};
use reqwest_eventsource::{Event, EventSource, RequestBuilderExt};
use serde::{Serialize, de::DeserializeOwned};

use crate::error::StreamError;
#[cfg(feature = "messages")]
use crate::messages::Messages;
use crate::{config::Config, error::Result};

#[derive(Debug, Clone, Default)]
pub struct Client {
    pub config: Config,
}

impl Client {
    pub fn new() -> Self {
        Client {
            ..Default::default()
        }
    }

    pub fn with_config(config: Config) -> Self {
        Client { config }
    }

    pub async fn post<I, O>(&self, body: I) -> Result<O>
    where
        I: Serialize,
        O: DeserializeOwned,
    {
        let client = reqwest::Client::new();

        let res = client
            .post(format!("{}/messages", self.config.base_url).as_str())
            .header("anthropic-version", self.config.api_version.clone())
            .header("x-api-key", self.config.api_key.clone())
            .json(&body)
            .send()
            .await?
            .error_for_status()?;

        let parsed = res.json::<O>().await?;
        Ok(parsed)
    }

    //  pub type BoxStream<T> = Pin<boxed::Box<dyn Stream<Item = T> + Send + Sync>>;
    pub async fn post_stream<'a, I, O>(
        &self,
        body: I,
    ) -> Result<BoxStream<'a, std::result::Result<O, StreamError>>>
    where
        I: Serialize,
        O: DeserializeOwned + std::marker::Send + 'static,
    {
        let client = reqwest::Client::new();

        let res = client
            .post(format!("{}/messages", self.config.base_url).as_str())
            .header(
                "anthropic-version",
                self.config.api_version.clone().as_str(),
            )
            .header("x-api-key", self.config.api_key.clone().as_str())
            .json(&body);

        Ok(stream(res.eventsource()?))
    }

    #[cfg(feature = "messages")]
    pub fn messages<'a>(&'a self) -> Messages<'a> {
        Messages::new(self)
    }
}

pub(crate) fn stream<O>(
    event_source: EventSource,
) -> BoxStream<'static, std::result::Result<O, StreamError>>
where
    O: DeserializeOwned + Send + 'static,
{
    let s = event_source
        .take_while(|ev| {
            future::ready(match ev {
                // Ok(Event::Event(message)) if message.data == "[DONE]" => false,
                Err(reqwest_eventsource::Error::StreamEnded) => false,

                _ => true,
            })
        })
        .filter_map(|ev| async move {
            println!("Event: {:?}", ev);
            match ev {
                Err(e) => Some(Err(StreamError::ReqwestEventstreamError(e))),

                Ok(Event::Message(message)) => {
                    if message.event == "ping" {
                        return None;
                    }

                    let parsed = serde_json::from_str::<O>(&message.data)
                        .map_err(StreamError::DeserializationError);

                    Some(parsed)
                }

                // Ok(Event::Comment(comment)) => {
                //     println!("Comment: {}", comment);
                //     None
                // }
                Ok(Event::Open) => None,
            }
        });

    Box::pin(s)
}
