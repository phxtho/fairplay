use libmdns::Responder;
use scrap::{Capturer, Display};
use std::io::ErrorKind::WouldBlock;
use std::io::Write;
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

fn capture(mut stream: TcpStream) -> std::io::Result<()> {
    let d = Display::primary().unwrap();
    let mut capturer = Capturer::new(d).unwrap();

    loop {
        match capturer.frame() {
            Ok(frame) => {
                stream.write_all(&frame).map_err(|e| {
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
