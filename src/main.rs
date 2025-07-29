mod soundcloud;
mod http;
use std::{
    error,
    net::{TcpListener},
};


#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {

    let mut mpd = mpd::Client::connect("127.0.0.1:6600").unwrap();
    mpd.volume(35).unwrap();
    mpd.push(mpd::Song { file: "http://127.0.0.1:6680".to_string(), ..Default::default() }).unwrap();

    let client = soundcloud_rs::Client::new().await?;
    let track = client.get_track_by_urn("soundcloud:tracks:469079373").await?;

    let audio_data = soundcloud::stream_to(&client, &track).await?;
    println!("Got {} bytes of audio", audio_data.len());

    let listener = TcpListener::bind("127.0.0.1:6680")?;
    println!("Serving on http://127.0.0.1:6680");


    loop {
        let (socket, _) = listener.accept()?;
        let data = audio_data.clone(); // cloning Vec<u8>
        tokio::spawn(async move {
            if let Err(e) = http::handle_client(socket, &data).await {
                eprintln!("Client error: {e}");
            }
        });
    }
}
