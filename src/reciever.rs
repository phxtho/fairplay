use mdns_sd::{ServiceDaemon, ServiceEvent};
use std::io::{BufReader, Read, Write};
use std::net::TcpStream;
use std::process::{Child, Command, Stdio};

const FRAME_RATE: &str = "60";
const PIXEL_FORMAT: &str = "bgr0";

fn spawn_ffplay(w: usize, h: usize) -> std::io::Result<Child> {
    Command::new("ffplay")
        .args(&[
            "-f",
            "rawvideo",
            "-pixel_format",
            PIXEL_FORMAT,
            "-video_size",
            &format!("{}x{}", w, h),
            "-framerate",
            FRAME_RATE,
            "-",
        ])
        .stdin(Stdio::piped())
        .spawn()
}

fn handle_connection(stream: TcpStream, w: usize, h: usize) -> std::io::Result<()> {
    let mut child = spawn_ffplay(w, h).expect("Failed to spawn ffplay");
    let mut out = child.stdin.take().expect("Child process has no stdin");

    let mut buffer: Vec<u8> = vec![0; w * 4];
    let mut reader = BufReader::new(stream);
    loop {
        let result = reader.read(&mut buffer);
        match result {
            Ok(n) => {
                if n != 0 {
                    out.write(&buffer).map_err(|e| {
                        eprintln!("Failed to write to ffplay: {}", e);
                        e
                    })?;
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
        }
    }

    Ok(())
}

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

                match handle_connection(stream, w, h) {
                    Ok(_) => println!("Connection closed"),
                    Err(e) => eprintln!("Error: {}", e),
                };
            }
            other_event => {
                println!("Other event: {:?}", other_event)
            }
        }
    }
}
