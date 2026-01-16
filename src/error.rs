#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("HTTP error: {0}")]
    EventsourceClientError(#[from] eventsource_client::Error),

    #[error("HTTP error: {0}")]
    ReqwestClientError(#[from] reqwest::Error),

    #[error("Serialization/Deserialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum StreamError {
    #[error("HTTP error: {0}")]
    EventsourceClientError(#[from] eventsource_client::Error),

    #[error("Deserialization error: {0}")]
    DeserializationError(#[from] serde_json::Error),
}