use futures_util::Stream;
use crate::client::{Licheszter, LicheszterResult};
use crate::models::board::Event;

impl Licheszter {
    /// Stream incoming events
    pub async fn stream_events(&self) -> LicheszterResult<impl Stream<Item = LicheszterResult<Event>>> {
        let addr = format!("{}/api/stream/event", self.base);
        let builder = self.client.get(&addr);
        self.to_model_stream(builder).await
    }
}