use http_body_util::Full;
use hyper::{body::Bytes, server::conn::http1, service::service_fn, Request, Response};
use std::{convert::Infallible, net::SocketAddr};
use tokio::net::TcpListener;

use hostname::get;
use libmdns::Responder;

const SERVICE_TYPE: &str = "_fairplay._tcp";
const PORT: u16 = 8080;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([0, 0, 0, 0], PORT));
    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);

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

    // loop to continuously accept incoming connections
    loop {
        let (stream, _) = listener.accept().await?;
        println!(
            "Accepted connection from: {:?}",
            stream.peer_addr().unwrap()
        );
        // Spawn a tokio task to serve multiple connections concurrently
        tokio::task::spawn(async move {
            // Bind the incoming connection to our service
            stream.try_write(buf)
        });
    }
}