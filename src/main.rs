use std::{io::Read, net::{TcpListener, TcpStream}};
use std::io::{BufRead, BufReader, Write};
use rsoundcloud::{
    SoundCloudClient,
    ResourceId,
    TracksApi,
    models::track::{Track},
};

fn talk_to_mpd() {
    let mut mpd = TcpStream::connect("127.0.0.1:6600").unwrap();
  let mut reader = BufReader::new(mpd.try_clone().unwrap());
    let mut welcome = String::new();
    reader.read_line(&mut welcome).unwrap();
    println!("MPD dit: {}", welcome.trim());

    mpd.write_all(b"status\n").unwrap();
    mpd.flush().unwrap();

    for line in reader.by_ref().lines() {
        let line = line.ok().unwrap();
        println!("Réponse: {}", line);
    }
}

// async fn test_soundcloud() {
    // let test = async {
    //     let client = SoundCloudClient::default().await.unwrap();
    //     let url = "https://soundcloud.com/soo0/lapix-silvia-320-1".to_string();
    //     let track = client.get_track(ResourceId::Url(url)).await.unwrap();
    //     track
    // };
// }

fn main() {
    talk_to_mpd();
}
