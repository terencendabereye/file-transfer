use std::net::IpAddr;

/// Link-local addresses (169.254/16, fe80::/10) found on this machine's interfaces —
/// the addresses a direct Ethernet/crossover link between two PCs would assign with no
/// DHCP server present. Informational only for now; auto-selecting one to bind/connect
/// to is future work once this has been exercised on real hardware.
pub fn list_link_local_addresses() -> Vec<String> {
    if_addrs::get_if_addrs()
        .unwrap_or_default()
        .into_iter()
        .filter(|iface| is_link_local(iface.ip()))
        .map(|iface| iface.ip().to_string())
        .collect()
}

fn is_link_local(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(v4) => v4.is_link_local(),
        IpAddr::V6(v6) => (v6.segments()[0] & 0xffc0) == 0xfe80,
    }
}
