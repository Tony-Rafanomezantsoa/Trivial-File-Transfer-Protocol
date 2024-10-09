use std::{fmt::Display, net::UdpSocket};

use tftppacket::{RRQPacket, WRQPacket};

fn main() -> Result<(), String> {
    let rrq_packet: Vec<u8> = vec![
        0x00, 0x01, // Opcode for RRQ
        // Filename: "example.txt"
        b'e', b'x', b'a', b'm', b'p', b'l', b'e', b'.', b't', b'x', b't', 0x00, b' ',
        // Mode: "octet"
        b'o', b'c', b't', b'e', b't', 0x00,
    ];

    println!("{:#?}", RRQPacket::parse(&rrq_packet)?);


    // let server_socket = UdpSocket::bind("0.0.0.0:69").map_err(|e| {
    //     format!("Unable to initialize a UDP server socket: {}", e)
    // })?;
    
    Ok(())

    
}
