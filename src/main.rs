use std::net::TcpStream;
use std::io::{self, Write};

/// Funktion zum Öffnen des TCP-Streams
fn open_connection(address: &str) -> io::Result<TcpStream> {
    TcpStream::connect(address)
}

/// Funktion zum Senden einer Nachricht über den TCP-Stream
fn send_message(stream: &mut TcpStream, message: &str) -> io::Result<()> {
    // Nachricht in Bytes umwandeln und senden
    stream.write_all(message.as_bytes())?;
    Ok(())
}

/// Funktion zum Setzen mehrerer Pixel
fn set_pixels(pixels: &[(u32, u32, &str)], stream: &mut TcpStream) -> io::Result<()> {
    let mut message = String::new();

    // Nachricht für alle Pixel zusammensetzen
    for &(x, y, color) in pixels {
        message.push_str(&format!("PX {} {} {}\n", x, y, color));
    }

    // Alle Pixeländerungen auf einmal senden
    send_message(stream, &message)
}

fn set_pixel(x: u32, y: u32, color: &str, stream: &mut TcpStream) -> io::Result<()> {
    // Erstelle die Nachricht im gewünschten Format
    let message = format!("PX {} {} {}\n", x, y, color);

    // Verwende die bereits implementierte Funktion send_message
    send_message(stream, &message)
}

fn main() -> io::Result<()> {
    let server_address = "table.apokalypse.email:1337"; // Die Adresse des Servers

    // Verbindung öffnen
    let mut stream = open_connection(server_address)?;

    // Beispielhafte Verwendung von set_pixels
    loop {
        let mut pixels = Vec::new();
        
        // Pixel sammeln (nur ein kleiner Teil für das Beispiel)
        for x in 0..=92 {
            for y in 0..=92 {
                pixels.push((x, y, "ff00ff"));
            }
        }
        
        // Sende alle Pixeländerungen in einer Nachricht
        set_pixels(&pixels, &mut stream)?;
        
    }
}

