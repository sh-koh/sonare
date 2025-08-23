use clap::{Parser, Subcommand};
use std::error;

use crate::backend;
use sonare_core::Provider;

#[cfg(feature = "soundcloud")]
use sonare_soundcloud::Soundcloud;
#[cfg(feature = "spotify")]
use sonare_spotify::Spotify;
#[cfg(feature = "youtube")]
use sonare_youtube::Youtube;

#[derive(Parser, Debug)]
#[command(name = "sonare")]
#[command(about = "A Mopidy-like in pure Rust", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Play the provided song
    Play {
        /// The URL of the song you want to play
        url: String,
    },

    /// Add new songs to the playlist
    Add {
        /// Songs to add to the playlist
        urls: Vec<String>,
    },
}

pub async fn cli_init() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Play { url } => {
            let _ = backend::play(url.as_str()).await;
        }
        Commands::Add { urls } => {
            println!("Adding to playlist: {:?}", urls);
            // backend::add(urls);
        }
    }
}

pub async fn str_to_provider(
    input: &str,
) -> Result<Box<dyn Provider + Send + Sync>, Box<dyn error::Error>> {
    #[cfg(feature = "soundcloud")]
    if input.starts_with("https://soundcloud.com/") || input.starts_with("soundcloud:tracks:") {
        dbg!("soundcloud!");
        return Ok(Box::new(Soundcloud));
    }

    #[cfg(feature = "spotify")]
    if input.starts_with("https://open.spotify.com/") || input.starts_with("spotify:track:") {
        dbg!("spotify!");
        return Ok(Box::new(Spotify));
    }

    #[cfg(feature = "youtube")]
    if input.starts_with("https://youtube.com/") || input.starts_with("https://youtu.be/") {
        dbg!("youtube!");
        return Ok(Box::new(Youtube));
    }

    Err("Unsupported provider or invalid URL/URN".into())
}
