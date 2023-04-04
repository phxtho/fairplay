use mdns_sd::{ServiceDaemon, ServiceEvent};
use std::io::{BufReader, Read, Write};
use std::net::TcpStream;
use std::process::{Command, Stdio};

pub fn run() {
    let mdns = ServiceDaemon::new().expect("Failed to create mdns daemon");
    //browse for services
    let service_type = "_fairplay._tcp.local."; // mdns_sd requires the full service type & it needs to end with a "."
    let reveiver = mdns
        .browse(service_type)
        .expect("Failed to browse for services");

    while let Ok(event) = reveiver.recv() {
        match event {
            ServiceEvent::ServiceResolved(info) => {
                let address = info.get_addresses().iter().next().unwrap();
                let port = info.get_port();
                let properties = info.get_properties();

                let w = properties
                    .get_property_val_str("width")
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();

                let h = properties
                    .get_property_val_str("height")
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();

                //make tcp connection
                let stream = TcpStream::connect(format!("{}:{}", address, port))
                    .expect("Failed to connect to server");

                handle_connection(stream, w, h);
            }
            other_event => {
                println!("Other event: {:?}", other_event)
            }
        }
    }
}

fn handle_connection(stream: TcpStream, w: usize, h: usize) {
    //spawn ffplay
    let child = Command::new("ffplay")
        .args(&[
            "-f",
            "rawvideo",
            "-pixel_format",
            "bgr0",
            "-video_size",
            &format!("{}x{}", w, h),
            "-framerate",
            "60",
            "-",
        ])
        .stdin(Stdio::piped())
        .spawn()
        .expect("This client requires ffplay.");

    let mut out = child.stdin.unwrap();

    let mut buffer: Vec<u8> = vec![0; w * 4]; // buffer size is one row of pixels
    let mut reader = BufReader::new(stream);
    loop {
        let result = reader.read(&mut buffer);
        match result {
            Ok(n) => {
                if n != 0 {
                    out.write(&buffer).expect("Failed to write to ffplay");
                }
            }
            Err(e) => {
                println!("Error: {}", e);

                break;
            }
        }
    }
}
