use core::error;
use std::{env, fmt::Display, fs::File, io::Read, net::UdpSocket};

use rand::Rng;
use tftppacket::{ERRORPacket, RRQPacket, RequestPacket, WRQPacket};

fn main() -> Result<(), String> {
    let server_socket = UdpSocket::bind("0.0.0.0:69")
        .map_err(|e| format!("Unable to initialize a UDP server socket: {}", e))?;

    println!("The TFTP server is running successfully...");

    loop {
        let mut request = [0_u8; 1024];

        let (_, client_addr) = match server_socket.recv_from(&mut request) {
            Ok(recv_info) => recv_info,
            Err(e) => {
                eprintln!(
                    "The server is unable to receive the TFTP request packet: {}",
                    e
                );
                continue;
            }
        };

        let request = match RequestPacket::parse(&request) {
            Ok(rq) => rq,
            Err(_) => {
                let error = ERRORPacket::create_custom_error_packet(
                    "The server received an invalid TFTP request packet.",
                );
                server_socket.send_to(&error, client_addr);
                continue;
            }
        };

        println!("CLIENT ADDR: {}", client_addr);

        println!("REQUEST: {:#?}", request);

        match request {
            RequestPacket::RRQ(rrq) => {
                let sub_server_socket = loop {
                    let server_tid: u16 = rand::thread_rng().gen_range(0..=65535);

                    match UdpSocket::bind(format!("0.0.0.0:{}", server_tid)) {
                        Ok(socket) => break socket,
                        Err(_) => continue,
                    }
                };

                if rrq.mode.to_lowercase() != "octet" {
                    let error = ERRORPacket::create_custom_error_packet(
                        "The server only supports the 'octet' mode for file transfers",
                    );
                    server_socket.send_to(&error, client_addr);
                    continue;
                }

                ///////////////////////////////////////////////////////
                //                  EXPERIMENTAL
                ///////////////////////////////////////////////////////
                

                let server_bin_path = match env::current_exe() {
                    Ok(server_bin_path) => server_bin_path,
                    Err(e) => {
                        let error = ERRORPacket::create_custom_error_packet(
                            format!("An error has occurred on the server: {}", e).as_str(),
                        );
                        server_socket.send_to(&error, client_addr);
                        continue;
                    }
                };

                let file_path = server_bin_path.parent().unwrap().join(&rrq.filename);

                let mut file = match File::open(file_path) {
                    Ok(f) => f,
                    Err(_) => continue, // **************** FILE NOT FOUND ERROR ******************
                };

                let mut buffer = [0_u8; 512];

                file.read(&mut buffer).unwrap();

                let mut data_packet: Vec<u8> = Vec::new();

                data_packet.extend_from_slice(&3_u16.to_be_bytes());

                data_packet.extend_from_slice(&1_u16.to_be_bytes());
                
                data_packet.extend_from_slice(&buffer);

                sub_server_socket.send_to(&data_packet, client_addr);
            }
            RequestPacket::WRQ(wrq) => {}
        }
    }
}
