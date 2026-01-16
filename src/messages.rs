use futures::stream::BoxStream;

use crate::{
    Client,
    error::{Result, StreamError},
    types::{CreateMessageParams, MessageStreamEvent},
};

#[derive(Debug, Clone)]
pub struct Messages<'a> {
    client: &'a Client,
}

impl<'a> Messages<'a> {
    pub fn new(client: &'a Client) -> Self {
        Messages { client }
    }

    pub async fn create(&self, _request: CreateMessageParams) -> Result<()> {
        Ok(())
    }

    pub async fn create_stream(
        self,
        mut request: CreateMessageParams,
    ) -> Result<BoxStream<'static, std::result::Result<MessageStreamEvent, StreamError>>> {
        // let mut request = request.clone();
        request.stream = Some(true);

        Ok(self.client.post_stream(request)?)
    }
}
