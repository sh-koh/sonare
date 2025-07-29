use std::net::{TcpStream};
use std::io::{Read, Write};
use std::error;


pub async fn handle_client(mut stream: TcpStream, audio_data: &[u8]) -> Result<(), Box<dyn error::Error>> {
    let request_buffer = &mut [0_u8; 8192];
    stream.read(request_buffer)?;

    stream.write_all(b"HTTP/1.1 200 OK\r\nContent-Type: audio/mpeg\r\n\r\n")?;
    stream.write_all(audio_data)?;

    Ok(())
}
