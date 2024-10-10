use std::{fmt::Display, net::UdpSocket};

use tftppacket::{ERRORPacket, RRQPacket, RequestPacket, WRQPacket};

fn main() -> Result<(), String> {
    let server_socket = UdpSocket::bind("0.0.0.0:69").map_err(|e| {
        format!("Unable to initialize a UDP server socket: {}", e)
    })?;

    println!("The TFTP server is running successfully...");

    loop {
        let mut request = [0_u8; 1024];
   
        let (_, client_addr) = match server_socket.recv_from(&mut request) {
            Ok(recv_info) => recv_info,
            Err(e) => {
                eprintln!("Unable to receive a TFTP request packet: {}", e);
                continue;
            }
        };

        let request = match RequestPacket::parse(&request) {
            Ok(rq) => rq,
            Err(_) => {
                let error = ERRORPacket::create_custom_error_packet(
                    "The server received an invalid TFTP request packet."
                );
                server_socket.send_to(&error, client_addr.clone());
                continue;
            },
        };

        println!("Client: {:#?}\n\nData: {:#?}", client_addr, request);
   }
}