use std::{fmt::Display, net::UdpSocket};

use tftppacket::{RRQPacket, WRQPacket};

fn main() -> Result<(), String> {
    let server_socket = UdpSocket::bind("0.0.0.0:69").map_err(|e| {
        format!("Unable to initialize a UDP server socket: {}", e)
    })?;

    todo!()
}