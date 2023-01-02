use std::net::TcpListener;

use libmdns::Responder;

const SERVICE_TYPE: &'static str = "_http._tcp";
const SERVICE_NAME: &'static str = "My Service";
const PORT: u16 = 5700;

fn main() {
    let listener =
        TcpListener::bind(format!("{}:{}", "127.0.0.1", PORT)).expect("Failed to bind to port");

    let port = listener.local_addr().unwrap().port();
    println!("Listening on port {}", port);

    let responder = Responder::new().unwrap();
    let _svc = responder.register(
        SERVICE_TYPE.to_owned(),
        SERVICE_NAME.to_owned(),
        port,
        &["path=/"],
    );

    loop {
        ::std::thread::sleep(::std::time::Duration::from_secs(10));
    }
}
