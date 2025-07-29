use crate::providers::AudioProvider;
use futures_util::StreamExt;
use gstreamer::prelude::*;
use gstreamer_app::AppSrc;

pub async fn play_stream(
    provider: Box<dyn AudioProvider + Send + Sync>,
    input: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    gstreamer::init()?;

    let url = provider.get_stream_url(input).await.unwrap();
    println!("Streaming from URL: {}", url);

    let pipeline = gstreamer::parse::launch(
        "appsrc name=mysrc format=3 ! decodebin ! audioconvert ! audioresample ! autoaudiosink",
    )?;
    let appsrc = pipeline
        .clone()
        .dynamic_cast::<gstreamer::Bin>()
        .unwrap()
        .by_name("mysrc")
        .unwrap()
        .downcast::<AppSrc>()
        .unwrap();

    appsrc.set_format(gstreamer::Format::Time);
    appsrc.set_caps(Some(
        &gstreamer::Caps::builder("audio/mpeg")
            .field("mpegversion", &1i32)
            .build(),
    ));

    pipeline.set_state(gstreamer::State::Playing)?;

    let resp = reqwest::get(&url).await?;
    let mut stream = resp.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        let gst_buffer = gstreamer::Buffer::from_slice(chunk.to_vec());
        appsrc.push_buffer(gst_buffer)?;
    }

    appsrc.end_of_stream()?;

    let bus = pipeline.bus().unwrap();
    for msg in bus.iter_timed(gstreamer::ClockTime::NONE) {
        use gstreamer::MessageView;
        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                eprintln!(
                    "GStreamer error from {:?}: {} ({:?})",
                    err.src().map(|s| s.path_string()),
                    err.error(),
                    err.debug()
                );
                break;
            }
            _ => (),
        }
    }

    pipeline.set_state(gstreamer::State::Null)?;

    Ok(())
}
