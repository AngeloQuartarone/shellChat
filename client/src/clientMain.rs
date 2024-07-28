use std::io::{self, BufRead, Read, Write};
use std::net::{TcpStream, UdpSocket};
use std::{str, thread};

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
                    Ok(stream) => {
                        println!("Connected to the server at {}", tcp_socket_address.trim());

                        // Interact with the echo server
                        handle_server(stream).expect("error in handle server function");/* .unwrap_or_else(|e| {
                            eprintln!("Error handling echo server: {}", e);
                        });*/
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

// fn handle_server(stream: &mut TcpStream) -> io::Result<()> {
//     let mut input = String::new();
//     let mut buffer = [0; 1024];

//     println!("Enter message to send to echo server:");

//     loop {
//         input.clear();
//         io::stdin().read_line(&mut input)?;
//         let message = input.trim();

//         if message.is_empty() {
//             println!("Empty message, closing connection.");
//             break;
//         }

//         // Send message to the echo server
//         stream.write_all(message.as_bytes())?;

//         // Receive echoed message
//         let bytes_read = stream.read(&mut buffer)?;
//         if bytes_read == 0 {
//             println!("Server closed connection.");
//             break;
//         }

//         let echoed_message = str::from_utf8(&buffer[..bytes_read]).unwrap_or("Invalid UTF-8 message received");
//         println!("Received from server: {}", echoed_message);
//     }

//     Ok(())
// }

fn handle_server(stream: TcpStream) -> io::Result<()> {
    let mut read_stream = stream.try_clone().expect("Failed to clone stream");
    let mut write_stream = stream.try_clone().expect("Failed to clone stream");

    let read_thread = thread::spawn(move || {
        let mut buffer = [0; 1024];
        loop {
            match read_stream.read(&mut buffer) {
                Ok(n) => {
                    if n == 0 {
                        println!("Server closed connection.");
                        break;
                    }
                    let echoed_message = String::from_utf8_lossy(&buffer[..n]);
                    println!("Server: {}", echoed_message);
                }
                Err(err) => {
                    eprintln!("Error reading from server: {}", err);
                    break;
                }
            }
        }
    });

    let write_thread = thread::spawn(move || {
        let stdin = io::stdin();
        let mut input = String::new();
        loop {
            input.clear();
            stdin.lock().read_line(&mut input).expect("Failed to read from stdin");
            let message = input.trim();
            if message.is_empty() {
                println!("Empty message, closing connection.");
                break;
            }
            match write_stream.write_all(message.as_bytes()) {
                Ok(_) => {}
                Err(err) => {
                    eprintln!("Error writing to server: {}", err);
                    break;
                }
            }
        }
    });

    write_thread.join().unwrap();
    // Read thread might still be running if the server didn't close the connection
    // but the user sent an empty message
    read_thread.join().unwrap();

    Ok(())
}
