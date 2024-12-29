use std::net::TcpStream;
use std::io::{self, Write};
use image::Pixel;

/// Funktion zum Öffnen des TCP-Streams
fn open_connection(address: &str) -> io::Result<TcpStream> {
    println!("opening stream");
    let stream = TcpStream::connect(address);
    println!("stream opened");
    return stream
}

/// Funktion zum Senden einer Nachricht über den TCP-Stream
fn send_message(stream: &mut TcpStream, message: &str) -> io::Result<()> {
    // Nachricht in Bytes umwandeln und senden
    stream.write_all(message.as_bytes())?;
    Ok(())
}

/// Funktion zum Setzen mehrerer Pixel
fn set_pixels(pixels: &Vec<((u32, u32), (u8, u8, u8))>, stream: &mut TcpStream) -> io::Result<()> {
    let mut message = String::new();

    // Nachricht für alle Pixel zusammensetzen
    for ((x, y), (r, g, b)) in pixels {
        message.push_str(&format!("PX {} {} {:X}{:X}{:X}\n", x, y, r, g, b));
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_address = "table.apokalypse.email:1337"; // Die Adresse des Servers
    let rgba_img = image::open("airbus.jpg")?.to_rgb8();

    let offset_x: u32 = 1700;
    let offset_y: u32 = 200;

    let (width, height) = rgba_img.dimensions();

    let mut pixels = Vec::new();

    for y in 0..height {
            for x in 0..width {
                let pixel = rgba_img.get_pixel(x, y); // Get pixel at (x, y)
                let rgba = pixel.channels(); // Extract RGBA channels
                pixels.push(((x + offset_x, y + offset_y), (rgba[0], rgba[1], rgba[2])));
            }
        }
    

    // Verbindung öffnen
    let mut stream = open_connection(server_address)?;

    // Beispielhafte Verwendung von set_pixels
    loop {
        // Sende alle Pixeländerungen in einer Nachricht
        set_pixels(&pixels, &mut stream)?;
        
    }
}

