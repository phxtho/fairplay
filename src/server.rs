use cfg_if::cfg_if;
use libmdns::Responder;
use scrap::{Capturer, Display, Frame};
use std::io::ErrorKind::WouldBlock;
use std::io::{Write, Result as IoResult};
use std::net::SocketAddr;
use std::net::{TcpListener, TcpStream};

const SERVICE_TYPE: &str = "_fairplay._tcp";
const PORT: u16 = 0; // 0 will let the OS choose a port

pub fn run() {
    let addr = SocketAddr::from(([0, 0, 0, 0], PORT));
    let listener = TcpListener::bind(addr).expect("Failed to bind to address");

    let hostname = hostname::get()
        .expect("Failed to get hostname")
        .into_string()
        .expect("Failed to convert hostname OsString to string");
    let d = Display::primary().expect("Failed to get primary display");

    let port = listener.local_addr().unwrap().port();
    println!("Listening on port {}", port);

    // Broadcast the service on the local network if the responder is dropped it will unregister the service
    let responder = Responder::new().expect("failed to intialize mdns responder");
    let _svc = responder.register(
        SERVICE_TYPE.to_owned(),
        hostname.to_owned(),
        port,
        &[
            &format!("width={}", d.width()),
            &format!("height={}", d.height()),
            "format=BGRA",
        ],
    );

    println!(
        "Registered {:?} with mdns responder under {:?}",
        hostname, SERVICE_TYPE
    );

    // loop to continuously accept incoming connections
    loop {
        let result = listener.accept();
        match result {
            Ok((stream, _)) => {
                println!("Accepted connection from: {:?}", stream.peer_addr());
                match capture(stream) {
                    Ok(_) => println!("Connection closed"),
                    Err(_) => {}
                };
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
        {}
    }
}

fn write_frame(
    frame: Frame,
    width: usize,
    height: usize,
    mut stream: &TcpStream,
) -> IoResult<()> {
    // removes end padding and writes to stream
    cfg_if! {
        if #[cfg(target_os = "macos")] {
            // garbage bytes are at the end of the frame on m1 macs
            let frame_size = 4 * width * height;
            stream.write_all(&frame[..frame_size])?;
            Ok(())
        } else {
            // garbage bytes are at the end of each row on linux
            let stride = frame.len() / height;
            let rowlen = 4 * width;

            for row in frame.chunks_exact(stride){
                let row = &row[..rowlen];
                stream.write_all(row)?;
            }
            Ok(())
        }
    }
}

fn capture(stream: TcpStream) -> IoResult<()> {
    let d = Display::primary().unwrap();
    let w = d.width();
    let h = d.height();
    let mut capturer = Capturer::new(d).unwrap();

    loop {
        match capturer.frame() {
            Ok(frame) => {
                write_frame(frame, w, h, &stream).map_err(|e| {
                    eprintln!("Failed to write to stream: {}", e);
                    e
                })?;
            }
            Err(ref e) if e.kind() == WouldBlock => {
                // Wait for the frame.
            }
            Err(_) => {
                // We're done here.
                break;
            }
        };
    }

    return Ok(());
}
