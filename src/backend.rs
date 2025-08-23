use crate::cli::str_to_provider;
use futures_util::StreamExt;
use gstreamer::prelude::*;
use gstreamer_app::AppSrc;
use std::error;

pub async fn play(track: &str) -> Result<(), Box<dyn error::Error>> {
    gstreamer::init()?;

    let provider = str_to_provider(track).await?;
    let url = provider.get_stream_url(track).await.unwrap();
    println!("Streaming from URL: {}", url);

    let pipeline = gstreamer::parse::launch(
        "appsrc name=mysrc format=3 ! decodebin ! audioconvert ! audioresample ! autoaudiosink",
    )?;

    let appsrc = pipeline
        .clone()
        .dynamic_cast::<gstreamer::Bin>()
        .ok()
        .and_then(|bin| bin.by_name("mysrc"))
        .and_then(|e| e.downcast::<AppSrc>().ok())
        .ok_or("Failed to get appsrc element")?;

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
        let buffer = gstreamer::Buffer::from_slice(chunk);
        appsrc.push_buffer(buffer)?;
    }

    appsrc.end_of_stream()?;

    let bus = pipeline.bus().unwrap();
    for msg in bus.iter_timed(gstreamer::ClockTime::NONE) {
        use gstreamer::MessageView::*;
        match msg.view() {
            Eos(..) => break,
            Error(err) => {
                eprintln!(
                    "GStreamer error from {:?}: {} ({:?})",
                    err.src().map(|s| s.path_string()),
                    err.error(),
                    err.debug()
                );
                break;
            }
            _ => {}
        }
    }

    pipeline.set_state(gstreamer::State::Null)?;
    Ok(())
}
