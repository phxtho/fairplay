use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};

use libmdns::Responder;

const SERVICE_TYPE: &str = "_http._tcp";
const SERVICE_NAME: &str = "My Service";
const PORT: u16 = 5700;

fn main() {
    let listener =
        TcpListener::bind(format!("{}:{}", "127.0.0.1", PORT)).expect("Failed to bind to port");

    let port = listener.local_addr().unwrap().port();
    println!("Listening on port {}", port);

    // if the responder is dropped it will unregister the service
    let responder = Responder::new().unwrap();
    let _svc = responder.register(
        SERVICE_TYPE.to_owned(),
        SERVICE_NAME.to_owned(),
        port,
        &["path=/"],
    );

    for stream in listener.incoming() {
        let stream = stream.expect("Failed to connect");
        read_stream(stream);
    }
}

fn read_stream(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream
        .read(&mut buffer)
        .expect("Failed to read from stream");

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}
