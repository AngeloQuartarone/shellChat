use std::net::UdpSocket;
use std::str;

fn main() {
    // Crea un socket UDP e lo lega a un indirizzo e porta specifici.
    // L'indirizzo "0.0.0.0:34254" significa che il client ascolterà su tutte le interfacce di rete sulla porta 34254.
    let socket = UdpSocket::bind("0.0.0.0:18200").expect("Couldn't bind to address");

    println!("Client in ascolto sulla porta 18200...");

    let mut buf = [0; 1024]; // Buffer per i dati ricevuti

    loop {
        // Ricevi i dati
        match socket.recv_from(&mut buf) {
            Ok((size, src)) => {
                // Converti i dati ricevuti in una stringa
                let msg = str::from_utf8(&buf[..size]).unwrap_or("Messaggio non valido");

                // Stampa le informazioni
                println!("Ricevuto {} byte da {}", size, src);
                println!("Messaggio: {}", msg);
            }
            Err(e) => {
                // Stampa un errore in caso di fallimento
                eprintln!("Errore nella ricezione dei dati: {}", e);
            }
        }
    }
}
