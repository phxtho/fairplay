use futures_util::{pin_mut, stream::StreamExt};
use mdns::{Error, Record, RecordKind};
use std::{net::IpAddr, time::Duration};

/// The hostname of the devices we are searching for.
/// Every Chromecast will respond to the service name in this example.
const SERVICE_NAME: &'static str = "_airplay._tcp.local";

#[async_std::main]
async fn main() -> Result<(), Error> {
    // Iterate through responses from each Cast device, asking for new devices every 15s
    let stream = mdns::discover::all(SERVICE_NAME, Duration::from_secs(15))?.listen();
    pin_mut!(stream);

    while let Some(Ok(response)) = stream.next().await {
        println!("Found device: {:?}", response.hostname());
        let addr = response.records().filter_map(self::to_ip_addr).next();

        if let Some(addr) = addr {
            println!("found cast device at {}", addr);
        } else {
            println!("cast device does not advertise address");
        }
    }

    Ok(())
}

fn addvertise_addr() ->  {

    mdns::
    
}

fn to_ip_addr(record: &Record) -> Option<IpAddr> {
    match record.kind {
        RecordKind::A(addr) => Some(addr.into()),
        RecordKind::AAAA(addr) => Some(addr.into()),
        _ => None,
    }
}
