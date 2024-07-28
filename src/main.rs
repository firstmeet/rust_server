mod packet;

use std::thread;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, UdpSocket};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Server Port
    #[arg(short='p', long)]
    port: u16,
}

fn main() {
    let args = Args::parse();
    thread::spawn(move || {
        start_tcp(args.port);
    });
    start_udp(args.port);
}

fn start_udp(port: u16) {
    let socket = UdpSocket::bind(("0.0.0.0", port)).expect("Failed to bind UDP socket");
    let mut buffer = [0; 1024];
    loop {
        let (size, src) = socket.recv_from(&mut buffer).expect("UDP read error");
        println!("UDP read {:?} from {:?}", &size, &src);
        match socket.send_to(&buffer[0..size], src) {
            Ok(size) => {
                println!("Sent data: {:?}", size);
            }
            Err(e) => {
                println!("Send error: {:?}", e);
            }
        }
    }
}

fn start_tcp(port: u16) {
    let listener = TcpListener::bind(("0.0.0.0", port)).expect("Failed to bind TCP listener");
    println!("Server listening on port {}", port);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection");
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("Connection failed: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    loop {
        let mut buffer = [0; 4];
        stream.read_exact(&mut buffer).expect("Read length error");
        let size = u32::from_be_bytes(buffer);
        let mut buffer = vec![0; size as usize];
        stream.read_exact(&mut buffer).expect("Read data error");
        println!("Received: {:?}", String::from_utf8(buffer.clone()).unwrap());
        // JSON parse
        let deserialized: packet::Packet = serde_json::from_slice(&buffer).expect("Failed to deserialize packet");
        println!("Deserialized: {:?}", deserialized);
        stream.set_nodelay(deserialized.no_delay).expect("Set no_delay error");
        stream.write_all(&deserialized.data).expect("Send data error");
    }
}