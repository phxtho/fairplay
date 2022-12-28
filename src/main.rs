use mdns_sd::{ServiceDaemon, ServiceEvent, ServiceInfo};
use std::{
    collections::HashMap,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

/// The hostname of the devices we are searching for.
const SERVICE_NAME: &'static str = "_airplay._tcp.local.";
const HOST_IPV4: &'static str = "127.0.0.1";
const PORT: u16 = 5200;

fn main() {
    let mdns = ServiceDaemon::new().expect("Failed to create mdns daemon");

    // Set up listener for incoming connections on localhost:5200.
    let listener = set_up_tcp_listener(HOST_IPV4, PORT, Box::new(read_stream));

    println!(
        "Listening on port: {}",
        listener.local_addr().unwrap().port()
    );
    // Register with the mdns daemon, which publishes the service.
    mdns.register(create_service())
        .expect("Failed to register our service");

    // Search for the service.
    let services = mdns
        .browse(SERVICE_NAME)
        .expect("Failed to search for services");

    // Iterate over the services and write to them
    services
        .iter()
        .for_each(|service_event| match service_event {
            ServiceEvent::ServiceResolved(service) => {
                println!("Found service: {:?}", service);
                write_stream(connect_to_service(service));
            }
            _ => {}
        });
}

fn create_service() -> ServiceInfo {
    let instance_name = "My instance";

    let host_name = "192.168.1.12.local.";
    let mut properties = HashMap::new();
    properties.insert("property_1".to_string(), "test".to_string());
    properties.insert("property_2".to_string(), "1234".to_string());

    let my_service = ServiceInfo::new(
        SERVICE_NAME,
        instance_name,
        host_name,
        HOST_IPV4,
        PORT,
        Some(properties),
    );
    let my_service = match my_service {
        Ok(service) => service,
        Err(e) => {
            panic!("Failed to create service info: {}", e);
        }
    };

    return my_service;
}

fn connect_to_service(service: ServiceInfo) -> TcpStream {
    let socket = TcpStream::connect(format!("{}:{}", service.get_hostname(), service.get_port()))
        .expect("Failed to connect to service");
    return socket;
}

fn read_stream(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream
        .read(&mut buffer)
        .expect("Failed to read from stream");

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}

fn write_stream(mut stream: TcpStream) {
    stream
        .write(b"Hello from the server")
        .expect("Failed to write to stream");
}

fn set_up_tcp_listener(host: &'static str, _: u16, _: Box<dyn Fn(TcpStream)>) -> TcpListener {
    let address = format!("{}:{}", host, 0); // 0 means that the OS will assign a port
    let listener = TcpListener::bind(address).expect("Failed to bind to port");
    listener
        .set_nonblocking(true)
        .expect("Cannot set non-blocking");

    // for stream in listener.incoming() {
    //     match stream {
    //         Ok(stream) => {
    //             handler(stream);
    //         }
    //         Err(e) => {
    //             // println!("Failed to connect to incoming stream: {}", e);
    //         }
    //     }
    // }

    return listener;
}
