use std::io::{self, Read, Write};
use std::net::{TcpStream, UdpSocket};
use std::str;

fn main() {
    let udp_socket = UdpSocket::bind("0.0.0.0:18200").expect("Couldn't bind to address");
    let mut buf = [0; 1024]; // Buffer per i dati ricevuti

    println!("Client in ascolto sulla porta 18200...");

    loop {
        // Receiving UDP messages
        match udp_socket.recv_from(&mut buf) {
            Ok((size, src)) => {
                // Convert the received data into a string
                let tcp_socket_address = str::from_utf8(&buf[..size]).unwrap_or("Messaggio non valido");

                // Print received data information
                println!("Ricevuto {} byte da {}", size, src);
                println!("Messaggio: {}", tcp_socket_address);

                // Connect to the server using TCP
                match TcpStream::connect(tcp_socket_address.trim()) {
                    Ok(mut stream) => {
                        println!("Connected to the server at {}", tcp_socket_address.trim());

                        // Interact with the echo server
                        handle_echo_server(&mut stream).unwrap_or_else(|e| {
                            eprintln!("Error handling echo server: {}", e);
                        });
                    }
                    Err(e) => {
                        eprintln!("Couldn't connect to server: {}", e);
                    }
                }
            }
            Err(e) => {
                // Print error message if receiving fails
                eprintln!("Errore nella ricezione dei dati: {}", e);
                std::process::exit(1);
            }
        }
    }
}

fn handle_echo_server(stream: &mut TcpStream) -> io::Result<()> {
    let mut input = String::new();
    let mut buffer = [0; 1024];

    println!("Enter message to send to echo server:");

    loop {
        input.clear();
        io::stdin().read_line(&mut input)?;
        let message = input.trim();

        if message.is_empty() {
            println!("Empty message, closing connection.");
            break;
        }

        // Send message to the echo server
        stream.write_all(message.as_bytes())?;

        // Receive echoed message
        let bytes_read = stream.read(&mut buffer)?;
        if bytes_read == 0 {
            println!("Server closed connection.");
            break;
        }

        let echoed_message = str::from_utf8(&buffer[..bytes_read]).unwrap_or("Invalid UTF-8 message received");
        println!("Received from server: {}", echoed_message);
    }

    Ok(())
}
