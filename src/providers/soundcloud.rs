use crate::providers::AudioProvider;
use soundcloud_rs::{
    query::TracksQuery,
    response::{Stream, StreamType, Track},
    Client,
};
use std::error::Error;

pub struct SoundCloud;

#[async_trait::async_trait]
impl AudioProvider for SoundCloud {
    async fn get_stream_url(&self, urn: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
        let client = Client::new().await.unwrap();
        let track: Track = client.get_track_by_urn(urn).await.unwrap();
        let transcoding = track
            .media
            .as_ref()
            .ok_or("Missing media")?
            .transcodings
            .as_ref()
            .ok_or("Missing transcodings")?
            .iter()
            .find(|t| {
                t.format.as_ref().and_then(|f| f.protocol.as_ref())
                    == Some(&StreamType::Progressive)
            })
            .ok_or("No progressive stream")?;

        let path = transcoding.url.as_ref().ok_or("Missing transcoding URL")?;
        let stream: Stream = Client::get_json(path, None, None::<&()>, &client.client_id)
            .await
            .unwrap();
        stream.url.ok_or("Missing resolved stream URL".into())
    }
}
