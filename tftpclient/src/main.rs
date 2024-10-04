use std::{fmt::format, net::UdpSocket, process, time::Duration};

use rand::Rng;
use utils::{ClientAction, ClientArgs};

mod utils;

fn main() {
    let client_args = match ClientArgs::build() {
        Ok(client_args) => client_args,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    let client_port: u16 = rand::thread_rng().gen_range(0..=65535);

    let client_socket = match UdpSocket::bind(format!("0.0.0.0:{}", client_port)) {
        Ok(socket) => socket,
        Err(e) => {
            eprintln!("Error: Unable to initialize UDP socket client: {}", e);
            process::exit(1);
        }
    };

    match client_args.action {
        ClientAction::Read => {
            let server_tid: u16;

            let mut read_request: Vec<u8> = Vec::new();
            // Read request (RRQ) opcode = 1
            read_request.extend_from_slice(&1_u16.to_be_bytes());
            // Read request (RRQ) filename
            read_request.extend_from_slice(client_args.filename.as_bytes());
            // Read request (RRQ) null byte
            read_request.push(0);
            // Read request (RRQ) mode
            read_request.extend_from_slice(client_args.mode.as_bytes());
            // Read request (RRQ) null byte
            read_request.push(0);

        }
        ClientAction::Write => {}
    }
}
