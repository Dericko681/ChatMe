use std::io;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

pub const DEFAULT_ADDR: &str = "127.0.0.1:7878";

/// Ensures an address string has a port, using 7878 as fallback.
pub fn parse_address(addr: &str) -> String {
    if addr.contains(':') {
        addr.to_string()
    } else {
        format!("{}:7878", addr)
    }
}

/// Binds a TcpListener to the specified address with logging.
pub fn bind_server(addr: &str) -> io::Result<TcpListener> {
    let parsed_addr = parse_address(addr);
    let listener = TcpListener::bind(&parsed_addr)?;
    println!("Server successfully bound to {}", parsed_addr);
    Ok(listener)
}

/// Attempts to connect to a server with retries and exponential backoff.
pub fn connect_with_retry(addr: &str, max_retries: u32) -> io::Result<TcpStream> {
    let parsed_addr = parse_address(addr);
    let mut delay = Duration::from_secs(1);

    for i in 0..max_retries {
        match TcpStream::connect(&parsed_addr) {
            Ok(stream) => {
                println!("Successfully connected to {}", parsed_addr);
                return Ok(stream);
            }
            Err(e) => {
                if i == max_retries - 1 {
                    return Err(e);
                }
                println!(
                    "Connection attempt {} failed. Retrying in {:?}...",
                    i + 1,
                    delay
                );
                thread::sleep(delay);
                delay *= 2; // Exponential backoff
            }
        }
    }
    Err(io::Error::new(
        io::ErrorKind::TimedOut,
        "Max retries reached",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_address() {
        assert_eq!(parse_address("localhost"), "localhost:7878");
        assert_eq!(parse_address("127.0.0.1"), "127.0.0.1:7878");
        assert_eq!(parse_address("127.0.0.1:8080"), "127.0.0.1:8080");
    }
}
