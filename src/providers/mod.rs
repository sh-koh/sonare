pub mod soundcloud;
pub mod spotify;
pub mod youtube;

use soundcloud::SoundCloud;
// use youtube::YouTube;
// use spotify::Spotify;

pub use self::soundcloud::*;
// pub use self::youtube::*;
// pub use self::spotify::*;

use std::error::Error;

#[async_trait::async_trait]
pub trait AudioProvider {
    async fn get_stream_url(&self, input: &str) -> Result<String, Box<dyn Error + Send + Sync>>;
}

#[derive(Debug)]
pub enum ProviderKind {
    SoundCloud,
    // YouTube,
    // Spotify,
}

impl ProviderKind {
    pub fn as_provider(&self) -> Box<dyn AudioProvider + Send + Sync> {
        match self {
            ProviderKind::SoundCloud => Box::new(SoundCloud),
            // ProviderKind::YouTube => Box::new(YouTube),
            // ProviderKind::Spotify => Box::new(Spotify),
        }
    }
}
