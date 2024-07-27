use if_addrs::get_if_addrs;
use std::{
    io::{Read, Write}, net::{IpAddr, TcpListener, TcpStream, UdpSocket}, thread::{self}, time::Duration
};

const UDP_SENDER_INTERVAL: u64 = 5000;
const TCP_PORT: i32 = 18200;

fn main() {
    let server_ip: IpAddr;
    match get_local_ip() {
        Some(ip) => {
            server_ip = ip;
            announcement_thread(ip, Duration::from_millis(UDP_SENDER_INTERVAL));
        }
        None => {
            eprintln!("Failed to obtain local IP address.");
            std::process::exit(1);
        }
    }

    let listener = TcpListener::bind(format!("{}:{}", server_ip, TCP_PORT)).unwrap_or_else(|_err| {
        eprintln!("Failed to bind to {}:{}", server_ip, TCP_PORT);
        std::process::exit(1);
    });

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(err) => {
                eprintln!("Connection failed: {}", err);
            }
        }
    }
}


fn handle_client(mut stream: TcpStream){
    loop {
        let mut read = [0; 1028];
        match stream.read(&mut read) {
            Ok(n) => {
                if n == 0 { 
                    // connection was closed
                    break;
                }
                stream.write(&read[0..n]).unwrap();
            }
            Err(err) => {
                panic!("{}", err);
            }
        }
    }
}






fn announcement_thread(ip: IpAddr, millis: Duration) {
    println!("[V] announcement thread started.");
    thread::spawn(move || {
        loop {
            udp_notice(ip.to_string());
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
