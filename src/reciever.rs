use mdns_sd::{ServiceDaemon, ServiceEvent};
use std::io::{BufReader, Read, Write};
use std::net::TcpStream;
use std::process::{Child, Command, Stdio};

const FRAME_RATE: &str = "60";
const PIXEL_FORMAT: &str = "bgr0";
const BYTES_PER_PIXEL: usize = 4;

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

fn read_exact_frame(reader: &mut dyn Read, buffer: &mut [u8]) -> std::io::Result<()> {
    let mut bytes_read = 0;
    while bytes_read < buffer.len() {
        match reader.read(&mut buffer[bytes_read..]) {
            Ok(n) => {
                if n == 0 {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::UnexpectedEof,
                        "Stream closed before reading a complete frame",
                    ));
                } else {
                    bytes_read += n;
                }
            }
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

fn handle_connection(stream: TcpStream, w: usize, h: usize) -> std::io::Result<()> {
    let mut child = spawn_ffplay(w, h)?;
    let mut out = child.stdin.take().expect("Child process has no stdin");

    let frame_size: usize = w * h * BYTES_PER_PIXEL;
    let mut buffer: Vec<u8> = vec![0; frame_size];
    let mut reader = BufReader::new(stream);
    loop {
        match read_exact_frame(&mut reader, &mut buffer) {
            Ok(()) => {
                out.write_all(&buffer)?;
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
