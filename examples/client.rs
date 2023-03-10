use std::io::{BufReader, Read};
use std::net::TcpStream;

use scrap::Display;

fn main() {
    //make tcp connection
    let stream = TcpStream::connect("127.0.0.1:8081").expect("Failed to connect to server");
    let d = Display::primary().unwrap();
    let (w, h) = (d.width(), d.height());

    // let child = Command::new("ffplay")
    //     .args(&[
    //         "-f",
    //         "rawvideo",
    //         "-pixel_format",
    //         "bgr0",
    //         "-video_size",
    //         &format!("{}x{}", w, h),
    //         "-framerate",
    //         "60",
    //         "-",
    //     ])
    //     .stdin(Stdio::piped())
    //     .spawn()
    //     .expect("This client requires ffplay.");

    // let mut out = child.stdin.unwrap();

    let mut buffer: Vec<u8> = Vec::new();
    let mut reader = BufReader::new(stream);

    loop {
        let result = reader.read(&mut buffer);
        match result {
            Ok(n) => {
                println!("n = {} ", n);
                println!("buffer.len() = {} ", buffer.len());
                if n != 0 {
                    let stride = n / h; // stride is the number bytes in a row
                    let rowlen = 4 * w;
                    for row in buffer.chunks(stride) {
                        let row = &row[..rowlen];
                        print!("{} ", row.len());
                    }
                    buffer.clear();
                }
            }
            Err(e) => {
                println!("Error: {}", e);

                break;
            }
        }
    }
}
