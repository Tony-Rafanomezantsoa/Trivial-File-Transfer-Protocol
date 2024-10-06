use std::net::UdpSocket;

use rand::Rng;
use tftppacket::{DATAPacket, ERRORPacket, RRQPacket};
use utils::{ClientAction, ClientArgs};

mod utils;

fn main() -> Result<(), String> {
    let client_args = ClientArgs::build()?;

    let client_tid: u16 = rand::thread_rng().gen_range(0..=65535);

    let client_socket = UdpSocket::bind(format!("0.0.0.0:{}", client_tid)).map_err(|e| {
        format!(
            "Unable to initialize an UDP socket client, please retry: {}",
            e
        )
    })?;

    match client_args.action {
        ClientAction::Read => {
            let rrq = RRQPacket::create_rrq_packet(&client_args.filename, &client_args.mode);

            client_socket
                .send_to(&rrq, format!("{}:69", client_args.remote_ip))
                .map_err(|e| format!("Unable to initialize a RRQ: {}", e))?;

            // Create a buffer for the first TFTP DATA packet
            // - Opcode: 2 bytes
            // - Block number: 2 bytes
            // - Data: 512 bytes
            let mut response = [0_u8; 516];

            let (_, remote_addr) = client_socket
                .recv_from(&mut response)
                .map_err(|e| format!("Unable to receive the first DATA packet: {}", e))?;

            let first_data_packet = match DATAPacket::parse(response) {
                Ok(packet) => packet,
                Err(e) => {
                    let error = ERRORPacket::create_custom_error_packet(&e);
                    client_socket.send_to(&error, remote_addr.clone());
                    return Err(format!("File transmission aborted due to an error: {}", e));
                }
            };

            if first_data_packet.block != 1 {
                let error = ERRORPacket::create_custom_error_packet(
                    "Received data with an invalid block number.",
                );
                client_socket.send_to(&error, remote_addr.clone());
                return Err(String::from("File transmission aborted due to an error: Received data with an invalid block number."));
            }

            let server_tid = remote_addr.port();
        }
        ClientAction::Write => {}
    }

    Ok(())
}
