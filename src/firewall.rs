// src/firewall.rs
use std::process::Command;

pub fn block_ip_iptables(ip: &str) {
    // iptables -A INPUT -s <ip> -j DROP
    // Requires root privileges
    let output = Command::new("iptables")
        .args(&["-A", "INPUT", "-s", ip, "-j", "DROP"])
        .output()
        .expect("Failed to run iptables");

    if output.status.success() {
        println!("✅ Blocked IP via iptables: {}", ip);
    } else {
        eprintln!("❌ Failed to block IP {}: {:?}", ip, output);
    }
}
