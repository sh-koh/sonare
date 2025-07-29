use std::{
    error,
};
use soundcloud_rs::{
    response::{Track, Stream, StreamType, Transcoding}, Client
};

pub async fn stream_to(
    client: &Client,
    track: &Track,
) -> Result<Vec<u8>, Box<dyn error::Error>> {
    let transcodings = track
        .media
        .as_ref()
        .unwrap()
        .transcodings
        .as_ref()
        .unwrap();

    let transcoding = transcodings
        .iter()
        .find(|t| t.format.as_ref().unwrap().protocol == Some(StreamType::Progressive))
        .ok_or("No progressive stream found")?;

    let url = get_stream_url(transcoding, client.client_id.as_str()).await?;
    let mut response = reqwest::get(&url).await?;
    let mut buffer = Vec::new();

    while let Some(chunk) = response.chunk().await? {
        buffer.extend_from_slice(&chunk);
    }

    Ok(buffer)
}


async fn get_stream_url(
    transcoding: &Transcoding,
    client_id: &str,
) -> Result<String, Box<dyn error::Error>> {
    let path = transcoding.url.as_ref().ok_or("Missing transcoding URL")?;
    let stream: Stream = Client::get_json(path, None, None::<&()>, client_id).await?;
    stream.url.ok_or("Missing resolved stream URL".into())
}
