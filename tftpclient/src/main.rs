use std::{env, fs::File, net::UdpSocket};

use rand::Rng;
use tftppacket::{ERRORPacket, RRQPacket, TFTPPacket, WRQPacket};
use utils::{ClientAction, ClientArgs};

mod utils;

fn main() -> Result<(), String> {
    let client_args = ClientArgs::build()?;

    let client_socket = loop {
        let client_tid: u16 = rand::thread_rng().gen_range(0..=65535);

        match UdpSocket::bind(format!("0.0.0.0:{}", client_tid)) {
            Ok(socket) => break socket,
            Err(_) => continue,
        };
    };

    match client_args.action {
        ClientAction::Read => {
            let rrq = RRQPacket::create_rrq_packet(&client_args.filename, "octet");

            client_socket
                .send_to(&rrq, format!("{}:69", client_args.remote_ip))
                .map_err(|e| format!("Unable to initialize a RRQ packet to the server: {}", e))?;

            // Create a buffer to store the first TFTP DATA packet
            // - Opcode: 2 bytes
            // - Block number: 2 bytes
            // - Data: 512 bytes
            let mut response = [0_u8; 516];

            let (_, server_addr) = client_socket
                .recv_from(&mut response)
                .map_err(|e| format!("Unable to receive a DATA packet from the server: {}", e))?;

            let first_data_packet = match TFTPPacket::parse(&response) {
                Ok(TFTPPacket::DATA(packet)) => packet,
                Ok(TFTPPacket::ERROR(err_packet)) => {
                    return Err(format!(
                        "File transmission aborted due to an error: {}",
                        err_packet.get_error_message()
                    ));
                }
                _ => {
                    let err_packet = ERRORPacket::IllegalTftpOperation;
                    client_socket.send_to(&err_packet.as_bytes(), server_addr);
                    return Err(format!(
                        "File transmission aborted due to an error: {}",
                        err_packet.get_error_message()
                    ));
                }
            };

            if first_data_packet.block != 1 {
                let err_packet = ERRORPacket::IllegalTftpOperation;
                client_socket.send_to(&err_packet.as_bytes(), server_addr);
                return Err(format!(
                    "File transmission aborted due to an error: {}",
                    err_packet.get_error_message()
                ));
            }

            let server_tid = server_addr.port();

            println!("SERVER PORT: {}", server_tid);

            println!("DATA: {:#?}", first_data_packet);
        }

        ClientAction::Write => {
            let current_dir = env::current_dir()
                .map_err(|e| format!("File transmission aborted due to an error: {}", e))?;

            let file = File::open(current_dir.join(&client_args.filename))
                .map_err(|e| format!("File transmission aborted due to an error: {}", e))?;

            let wrq = WRQPacket::create_wrq_packet(&client_args.filename, "octet");

            client_socket
                .send_to(&wrq, format!("{}:69", client_args.remote_ip))
                .map_err(|e| format!("Unable to initialize a WRQ packet to the server: {}", e))?;

            // Create a buffer to store the
            // first TFTP ACK packet for write
            // - Opcode: 2 bytes
            // - Block number: 2 bytes
            let mut response = [0_u8; 4];

            let (_, server_addr) = client_socket
                .recv_from(&mut response)
                .map_err(|e| format!("Unable to receive a ACK packet from the server: {}", e))?;

            let ack_packet_for_write = match TFTPPacket::parse(&response) {
                Ok(TFTPPacket::ACK(packet)) => packet,
                Ok(TFTPPacket::ERROR(err_packet)) => {
                    return Err(format!(
                        "File transmission aborted due to an error: {}",
                        err_packet.get_error_message()
                    ));
                }
                _ => {
                    let err_packet = ERRORPacket::IllegalTftpOperation;
                    client_socket.send_to(&err_packet.as_bytes(), server_addr);
                    return Err(format!(
                        "File transmission aborted due to an error: {}",
                        err_packet.get_error_message()
                    ));
                }
            };

            if ack_packet_for_write.block != 0 {
                let err_packet = ERRORPacket::IllegalTftpOperation;
                client_socket.send_to(&err_packet.as_bytes(), server_addr);
                return Err(format!(
                    "File transmission aborted due to an error: {}",
                    err_packet.get_error_message()
                ));
            }

            let server_tid = server_addr.port();
        }
    }

    Ok(())
}
