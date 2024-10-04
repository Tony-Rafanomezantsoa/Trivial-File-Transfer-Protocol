use std::{net::UdpSocket, process};

use rand::Rng;
use utils::ClientArgs;

mod utils;

fn main() {
    let client_args = match ClientArgs::build() {
        Ok(client_args) => client_args,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    let client_port = rand::thread_rng().gen_range(0..=65535);

    let client_socket = match UdpSocket::bind(format!("0.0.0.0:{}", client_port)) {
        Ok(socket) => socket,
        Err(e) => {
            eprintln!("Error: Unable to initialize UDP socket client: {}", e);
            process::exit(1);
        }
    };
}