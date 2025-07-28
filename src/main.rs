use rsoundcloud::{
    SoundCloudClient,
    ResourceId,
    TracksApi,
    models::track::{Track},
};

#[tokio::main]
async fn main() {
    let test = async {
        let client = SoundCloudClient::default().await.unwrap();
        let url = "https://soundcloud.com/soo0/lapix-silvia-320-1".to_string();
        let track = client.get_track(ResourceId::Url(url)).await.unwrap();
        track
    };

    println!("{:#?}", test.await);
}
