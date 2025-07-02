// same imports as before
use rustls::{ServerConfig, ServerConnection, StreamOwned};
use std::sync::Arc;

fn handle_tls_connection(mut stream: StreamOwned<ServerConnection, TcpStream>) {
    let mut buffer = [0u8; 1024];
    let n = stream.read(&mut buffer).unwrap();
    let command = std::str::from_utf8(&buffer[..n]).unwrap().trim();

    match command {
        "/create" => {
            // handle create
        }
        "/split" => {
            // handle split
        }
        "/destroy" => {
            // handle destroy
        }
        _ => {
            // unknown command
        }
    }

    stream.write_all(b"OK\n").unwrap();
    stream.flush().unwrap();
}
