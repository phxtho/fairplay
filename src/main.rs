use std::net::SocketAddr;
use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
};

use hostname::get;
use libmdns::Responder;

const SERVICE_TYPE: &str = "_fairplay._tcp";
const PORT: u16 = 8080;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([0, 0, 0, 0], PORT));
    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);

    broadcast_service();

    // loop to continuously accept incoming connections
    loop {
        let (stream, _) = listener.accept().await?;
        println!(
            "Accepted connection from: {:?}",
            stream.peer_addr().unwrap()
        );

        capture(stream).await;
    }
}

async fn capture(mut stream: TcpStream) {
    use scrap::{Capturer, Display};
    use std::io::ErrorKind::WouldBlock;
    let d = Display::primary().unwrap();
    let (w, h) = (d.width(), d.height());

    let mut capturer = Capturer::new(d).unwrap();

    loop {
        match capturer.frame() {
            Ok(frame) => {
                stream.write_all(&frame).await.unwrap();
            }
            Err(ref e) if e.kind() == WouldBlock => {
                // Wait for the frame.
            }
            Err(_) => {
                // We're done here.
                break;
            }
        }
    }
}

fn broadcast_service() {
    // Get the hostname of the device
    let hostname = get().unwrap().into_string().unwrap();

    /*
    Broadcast the service on the local network
    if the responder is dropped it will unregister the service
    */
    let responder = Responder::new().unwrap();
    let _svc = responder.register(
        SERVICE_TYPE.to_owned(),
        hostname.to_owned(),
        PORT,
        &["path=/"],
    );

    println!(
        "Registered {:?} with mdns responder under {:?}",
        hostname, SERVICE_TYPE
    );
}
