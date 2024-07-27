use if_addrs::get_if_addrs;
use std::{
    net::{IpAddr, UdpSocket},
    thread::{self},
    time::Duration,
};

fn main() {
    match get_local_ip() {
        Some(ip) => {
            scheduled_thread(ip);

            loop {
                thread::sleep(Duration::from_secs(60));
            }
        }
        None => println!("nessun IP trovato :("),
    }
}

fn scheduled_thread(ip: IpAddr) {
    thread::spawn(move || {
        let millis = Duration::from_millis(5000);

        loop {
            udp_notice(ip.to_string());
            println!("message sent!");
            thread::sleep(millis);
        }
    });
}

fn udp_notice(ip: String) {
    let broadcast_addr: String = "255.255.255.255:18200".to_string();
    let address_to_send: String = format!("{}:18200", ip);
    let socket = UdpSocket::bind("0.0.0.0:0").expect("bind failed");
    socket
        .set_broadcast(true)
        .expect("setting broadcast failed");

    socket
        .send_to(address_to_send.as_bytes(), broadcast_addr)
        .expect("send broadcast failed");
}

fn get_local_ip() -> Option<IpAddr> {
    let if_addrs = get_if_addrs().ok()?;
    for iface in if_addrs {
        if iface.is_loopback() {
            continue;
        }
        if let IpAddr::V4(ipv4) = iface.ip() {
            return Some(IpAddr::V4(ipv4));
        }
    }
    None
}
