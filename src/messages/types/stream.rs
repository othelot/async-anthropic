use serde::{Deserialize, Serialize};

use crate::error::StreamError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessagesStreamEvent {}

pub type MessagesStream = std::pin::Pin<
    Box<dyn futures::Stream<Item = std::result::Result<MessagesStreamEvent, StreamError>> + Send>,
>;
