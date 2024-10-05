use std::{fmt::format, net::UdpSocket, process, time::Duration};

use rand::Rng;
use utils::{ClientAction, ClientArgs};
use tftppacket::{DataPacket, RRQPacket};

mod utils;

fn main() -> Result<(), String> {
    let client_args = ClientArgs::build()?;

    let client_port: u16 = rand::thread_rng().gen_range(0..=65535);

    let client_socket = UdpSocket::bind(format!("0.0.0.0:{}", client_port)).map_err(|e| {
        format!(
            "Unable to initialize an UDP socket client, please retry: {}",
            e
        )
    })?;

    match client_args.action {
        ClientAction::Read => {
            let rrq = RRQPacket::create_rrq_packet(&client_args.filename, &client_args.mode)?;

            client_socket
                .send_to(&rrq, format!("{}:69", client_args.remote_ip))
                .map_err(|e| format!("Unable to initialize a RRQ: {}", e))?;

            // Create a buffer for a TFTP data packet
            // The packet structure includes:
            // - Opcode: 2 bytes (indicates the packet type)
            // - Block number: 2 bytes (identifies the data block)
            // - Data: 512 bytes (the actual data being transmitted)
            let mut response = [0_u8; 516];

            let (_, remote_addr) = client_socket
                .recv_from(&mut response)
                .map_err(|e| format!("Unable to receive data packet: {}", e))?;

            let first_data_packet = match DataPacket::parse(&response) {
                Ok(packet) => packet,
                // INVALID DATA PACKET
                // Send an "ERR" packet to the remote server and exit the program
                Err(e) => todo!(),
            };

            if first_data_packet.block != 1 {
                // INVALID DATA PACKET
                // Send an "ERR" packet to the remote server and exit the program
                todo!()
            }

            let server_tid = remote_addr.port();
        }
        ClientAction::Write => {}
    }

    Ok(())
}
