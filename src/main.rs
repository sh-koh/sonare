mod backend;
mod http;
mod providers;

use providers::ProviderKind;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = ProviderKind::SoundCloud.as_provider();
    let input = "soundcloud:tracks:1424212474"; // FIXME: hardcoded for now.
    backend::play_stream(provider, input).await?;
    Ok(())
}
