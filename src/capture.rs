// src/capture.rs

use crate::db::Database;
use crate::TrafficData; // Arc<Mutex<HashMap<String, (usize, String)>>>

use etherparse::{PacketHeaders, NetHeaders, TransportHeader};
use pcap::Capture;
use std::net::{Ipv4Addr, Ipv6Addr};
use time::{OffsetDateTime, format_description::well_known::Rfc3339};

pub fn capture_packets(
    traffic_data: TrafficData,
    db_arc: std::sync::Arc<std::sync::Mutex<Database>>,
) {
    let device = pcap::Device::list()
        .unwrap()
        .into_iter()
        .next()
        .expect("No devices found!");

    let mut cap = Capture::from_device(device)
        .unwrap()
        .promisc(true)
        .timeout(500)
        .open()
        .unwrap();

    while let Ok(packet) = cap.next() {
        if let Ok(parsed) = PacketHeaders::from_ethernet_slice(packet.data) {
            let mut map_guard = traffic_data.lock().unwrap();

            if let Some(net) = parsed.net {
                let (src_str, dst_str) = match net {
                    NetHeaders::Ipv4(ipv4, _) => (
                        Ipv4Addr::from(ipv4.source).to_string(),
                        Ipv4Addr::from(ipv4.destination).to_string(),
                    ),
                    NetHeaders::Ipv6(ipv6, _) => (
                        Ipv6Addr::from(ipv6.source).to_string(),
                        Ipv6Addr::from(ipv6.destination).to_string(),
                    ),
                };

                let proto = match parsed.transport {
                    Some(TransportHeader::Tcp(_)) => "TCP",
                    Some(TransportHeader::Udp(_)) => "UDP",
                    Some(TransportHeader::Icmpv4(_)) => "ICMP",
                    Some(TransportHeader::Icmpv6(_)) => "ICMPv6",
                    _ => "Other",
                };

                let conn_str = format!("{} -> {}", src_str, dst_str);
                let entry = map_guard.entry(conn_str.clone()).or_insert((0, proto.to_string()));
                entry.0 += packet.header.len as usize;

                // Insert into DB
                let db_guard = db_arc.lock().unwrap();
                let timestamp = OffsetDateTime::now_utc()
                    .format(&Rfc3339)
                    .unwrap(); // "2025-02-01T12:34:56Z"

                let _ = db_guard.insert_log(&timestamp, &conn_str, proto, packet.header.len as u64);
            }
        }
    }
}
