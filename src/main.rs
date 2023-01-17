use http_body_util::Full;
use hyper::{body::Bytes, server::conn::http1, service::service_fn, Request, Response};
use std::{convert::Infallible, net::SocketAddr};
use tokio::net::TcpListener;

use libmdns::Responder;

const SERVICE_TYPE: &str = "_fairdrop._tcp";
const SERVICE_NAME: &str = "My Service";
const PORT: u16 = 5700;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], PORT));
    let listener = TcpListener::bind(addr).await?;

    /*
    Broadcast the service on the local network
    if the responder is dropped it will unregister the service
    */
    let responder = Responder::new().unwrap();
    let _svc = responder.register(
        SERVICE_TYPE.to_owned(),
        SERVICE_NAME.to_owned(),
        PORT,
        &["path=/"],
    );

    println!(
        "Registered {:?} with mdns responder under {:?}",
        SERVICE_NAME, SERVICE_TYPE
    );

    // loop to continuously accept incoming connections
    loop {
        let (stream, _) = listener.accept().await?;
        // Spawn a tokio task to serve multiple connections concurrently
        tokio::task::spawn(async move {
            // Bind the incoming connection to our `hello` service
            if let Err(err) = http1::Builder::new()
                // `service_fn` converts our function in a `Service`
                .serve_connection(stream, service_fn(hello))
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}

async fn hello(_: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
}