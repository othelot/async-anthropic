pub mod types;

use crate::{
    Client,
    error::Result,
    messages::types::{CreateMessages, stream::MessagesStream},
};

#[derive(Debug, Clone)]
pub struct Messages<'a> {
    client: &'a Client,
}

impl<'a> Messages<'a> {
    pub fn new(client: &'a Client) -> Self {
        Messages { client }
    }

    pub async fn create_stream(self, request: CreateMessages) -> Result<MessagesStream> {
        Ok(self.client.post_stream(request)?)
    }
}
