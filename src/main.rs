use std::{
    collections::HashMap,
    net::{SocketAddr, UdpSocket},
};

use clap::Parser;

#[derive(Parser)]
struct Args {
    /// bind the service to this tcp port, default 5555
    #[arg(short, long, default_value = "5555")]
    port: u16,
}

fn main() {
    let args = Args::parse();
    let s = format!("0.0.0.0:{}", args.port)
        .parse::<SocketAddr>()
        .unwrap();
    println!("Listening to {s}");

    let socket = UdpSocket::bind(s).unwrap();
    let mut buf = [0u8; 1000];
    let mut db: HashMap<String, String> = HashMap::new();
    loop {
        let (count, from) = socket.recv_from(&mut buf).unwrap();
        let filled_buf = &buf[..count];
        println!("{from} - received {count} bytes,");
        // Ignore non utf-8 strings
        if let Ok(query) = String::from_utf8(filled_buf.to_vec()) {
            println!("{from} - {query}");
            if let Some(idx) = query.find('=') {
                // put
                let key = &query[0..idx];
                let value = &query[(idx + 1)..];
                db.insert(key.to_string(), value.to_string());
            } else {
                // get
                let value = if &query == "version" {
                    "UDP v0.1"
                } else {
                    db.get(&query).map(String::as_str).unwrap_or("")
                };
                let message = format!("{query}={value}");
                println!("{from} - sending {message}");
                let _ = socket.send_to(&message.as_bytes(), from);
            }
        } else {
            eprintln!("{from} - non utf8 bytes");
        }
    }
}
