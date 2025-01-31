// src/detection.rs
use std::collections::{HashMap, HashSet};
use std::time::{Instant, Duration};

pub const DDOS_BYTES_LIMIT: usize = 10_000_000;  // 10MB
pub const WINDOW_SECONDS: u64 = 5;
pub const PORT_SCAN_LIMIT: usize = 10;

pub struct IpStats {
    pub bytes_in_window: usize,
    pub last_update: Instant,
    pub ports_seen: HashSet<u16>,
}

impl IpStats {
    pub fn new() -> Self {
        IpStats {
            bytes_in_window: 0,
            last_update: Instant::now(),
            ports_seen: HashSet::new(),
        }
    }
}

pub fn check_ddos(ip_stats: &mut IpStats, packet_bytes: usize) -> bool {
    let now = Instant::now();
    if now.duration_since(ip_stats.last_update) > Duration::from_secs(WINDOW_SECONDS) {
        // Reset
        ip_stats.bytes_in_window = 0;
        ip_stats.ports_seen.clear();
        ip_stats.last_update = now;
    }

    // Add data
    ip_stats.bytes_in_window += packet_bytes;

    // Check threshold
    ip_stats.bytes_in_window > DDOS_BYTES_LIMIT
}

pub fn check_port_scan(ip_stats: &mut IpStats, port: u16) -> bool {
    ip_stats.ports_seen.insert(port);
    ip_stats.ports_seen.len() > PORT_SCAN_LIMIT
}
