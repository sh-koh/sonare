use std::error;

use async_trait::async_trait;

#[derive(Default)]
pub struct Sonare {
    pub playing_track: Option<SonareTrack>,
    pub following_tracks: Vec<SonareTrack>,
    pub looping: SonarePlayerLoop,
    pub random: bool,
    pub state: SonarePlayerState,
}

pub struct SonareTrack {
    pub title: String,
    pub artist: String,
    // pub cover: Image,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SonarePlayerLoop {
    Track,
    Playlist,
    None,
}

impl Default for SonarePlayerLoop {
    fn default() -> Self {
        SonarePlayerLoop::None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SonarePlayerState {
    Playing,
    Paused,
    NoTrack,
}

impl Default for SonarePlayerState {
    fn default() -> Self {
        SonarePlayerState::NoTrack
    }
}

impl Sonare {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
pub trait Provider {
    async fn get_stream_url(
        &self,
        input: &str,
    ) -> Result<String, Box<dyn error::Error + Send + Sync>>;
}
