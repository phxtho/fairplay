use hostname::get;
use libmdns::Responder;
use scrap::{Capturer, Display};
use std::io::ErrorKind::WouldBlock;
use std::io::Write;
use std::net::SocketAddr;
use std::net::{TcpListener, TcpStream};

const SERVICE_TYPE: &str = "_fairplay._tcp";
const PORT: u16 = 8081;

fn main() {
    let addr = SocketAddr::from(([0, 0, 0, 0], PORT));
    let listener = TcpListener::bind(addr).unwrap();
    println!("Listening on http://{}", addr);
    let hostname = get().unwrap().into_string().unwrap();
    let d = Display::primary().unwrap();

    // Broadcast the service on the local network if the responder is dropped it will unregister the service
    let responder = Responder::new().expect("failed to intialize mdns responder");
    let _svc = responder.register(
        SERVICE_TYPE.to_owned(),
        hostname.to_owned(),
        PORT,
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
                capture(stream)
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
        {}
    }
}

fn capture(mut stream: TcpStream) {
    let d = Display::primary().unwrap();
    let mut capturer = Capturer::new(d).unwrap();

    loop {
        match capturer.frame() {
            Ok(frame) => {
                stream.write_all(&frame).expect("Failed to write to stream");
                stream.flush().expect("Failed to flush stream");
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
}
