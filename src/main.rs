use std::{
    io::Read,
    net::{TcpListener, TcpStream},
    thread,
};

use libmdns::Responder;

const SERVICE_TYPE: &str = "_fairdrop._tcp";
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
    println!(
        "Registered {:?} with mdns responder under {:?}",
        SERVICE_NAME, SERVICE_TYPE
    );

    // process incoming connections on a separate thread
    thread::spawn(move || {
        for stream in listener.incoming() {
            let stream = stream.expect("Failed to connect");
            handle_connection(stream);
        }
    });

    loop {
        thread::sleep(std::time::Duration::from_secs(1));
    }
}

fn handle_connection(mut stream: TcpStream) {
    // allocate a buffer to read data into
    let mut buffer = [0; 1024];

    // read data from stream to buffer
    stream
        .read(&mut buffer)
        .expect("Failed to read from stream");

    // parse the request
    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut req = httparse::Request::new(&mut headers);
    let res = req.parse(&buffer).unwrap();

    println!("Res: {:?}", res);
}
