// src-tauri/src/capture.rs

use crate::db::Database;
use etherparse::{PacketHeaders, NetHeaders, TransportHeader};
use pcap::Capture;
use std::net::{Ipv4Addr, Ipv6Addr};
use time::{OffsetDateTime, format_description::well_known::Rfc3339};
use std::sync::{Arc, Mutex};

pub fn start_capture(db_arc: Arc<Mutex<Database>>) {
    // Open your pcap device
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

    // Simplistic capture loop
    while let Ok(packet) = cap.next() {
        if let Ok(parsed) = PacketHeaders::from_ethernet_slice(packet.data) {
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

                // Insert into DB
                let db_guard = db_arc.lock().unwrap();
                let timestamp = OffsetDateTime::now_utc()
                    .format(&Rfc3339)
                    .unwrap();

                let _ = db_guard.insert_log(&timestamp, &conn_str, proto, packet.header.len as u64);
            }
        }
    }
}
