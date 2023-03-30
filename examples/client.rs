use std::io::{BufReader, Read, Write};
use std::net::TcpStream;
use std::process::{Command, Stdio};

use scrap::Display;

fn main() {
    //make tcp connection
    let stream = TcpStream::connect("127.0.0.1:8081").expect("Failed to connect to server");
    let d = Display::primary().unwrap();
    let (w, h) = (d.width(), d.height());

    let child = Command::new("ffplay")
        .args(&[
            "-f",
            "rawvideo",
            "-pixel_format",
            "bgr0",
            "-video_size",
            &format!("{}x{}", w, h),
            "-framerate",
            "60",
            "-",
        ])
        .stdin(Stdio::piped())
        .spawn()
        .expect("This client requires ffplay.");

    let mut out = child.stdin.unwrap();

    let mut buffer: Vec<u8> = vec![0; w * 4]; // buffer size is one row of pixels
    let mut reader = BufReader::new(stream);
    loop {
        let result = reader.read(&mut buffer);
        match result {
            Ok(n) => {
                if n != 0 {
                    out.write(&buffer).expect("Failed to write to ffplay");
                }
            }
            Err(e) => {
                println!("Error: {}", e);

                break;
            }
        }
    }
}
