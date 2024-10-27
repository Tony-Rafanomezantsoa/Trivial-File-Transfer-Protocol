use std::{
    env,
    fs::{File, OpenOptions},
    io::{Read, Write},
    net::UdpSocket,
    path::Path,
};

use rand::Rng;
use tftppacket::{ACKPacket, DATAPacket, ERRORPacket, RRQPacket, TFTPPacket, WRQPacket};
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

            let (recv_packet_len, server_addr) = client_socket
                .recv_from(&mut response)
                .map_err(|e| format!("Unable to receive a DATA packet from the server: {}", e))?;

            // Ensure connection with the server
            client_socket
                .connect(server_addr)
                .map_err(|e| format!("Unable to etablish a connection with the server: {}", e))?;

            let first_data_packet = match TFTPPacket::parse(&response[..recv_packet_len]) {
                Ok(TFTPPacket::DATA(packet)) => packet,
                Ok(TFTPPacket::ERROR(err_packet)) => {
                    return Err(format!(
                        "File transmission aborted due to an error: {}",
                        err_packet.get_error_message()
                    ));
                }
                _ => {
                    let err_packet = ERRORPacket::IllegalTftpOperation;
                    client_socket.send(&err_packet.as_bytes());
                    return Err(format!(
                        "File transmission aborted due to an error: {}",
                        err_packet.get_error_message()
                    ));
                }
            };

            if first_data_packet.block != 1 {
                let err_packet = ERRORPacket::IllegalTftpOperation;
                client_socket.send(&err_packet.as_bytes());
                return Err(format!(
                    "File transmission aborted due to an error: {}",
                    err_packet.get_error_message()
                ));
            }

            let working_dir = env::current_dir()
                .map_err(|e| format!("File transmission aborted due to an error: {}", e))?;

            let filename = Path::new(&client_args.filename).file_name().unwrap();

            let mut file = OpenOptions::new()
                .create_new(true)
                .append(true)
                .open(working_dir.join(filename))
                .map_err(|e| format!("File transmission aborted due to an error: {}", e))?;

            let write_bytes = file
                .write(first_data_packet.get_data())
                .map_err(|e| format!("File transmission aborted due to an error: {}", e))?;

            let ack = ACKPacket {
                block: first_data_packet.block,
            };

            client_socket
                .send(&ack.as_bytes())
                .map_err(|e| format!("File transmission aborted due to an error: {}", e))?;

            if write_bytes < 512 {
                println!("Download completed!");
                println!("File path: {}", working_dir.join(filename).display());
                return Ok(());
            }

            let mut last_block_number = first_data_packet.block;

            loop {
                // Create a buffer to store the next
                // TFTP DATA packet (516 bytes)
                let mut response = [0_u8; 516];

                let recv_packet_len = client_socket
                    .recv(&mut response)
                    .map_err(|e| format!("File transmission aborted due to an error: {}", e))?;

                let data_packet = match TFTPPacket::parse(&response[..recv_packet_len]) {
                    Ok(TFTPPacket::DATA(packet)) => packet,
                    Ok(TFTPPacket::ERROR(err_packet)) => {
                        return Err(format!(
                            "File transmission aborted due to an error: {}",
                            err_packet.get_error_message()
                        ));
                    }
                    _ => {
                        let err_packet = ERRORPacket::IllegalTftpOperation;
                        client_socket.send(&err_packet.as_bytes());
                        return Err(format!(
                            "File transmission aborted due to an error: {}",
                            err_packet.get_error_message()
                        ));
                    }
                };

                if data_packet.block != (last_block_number + 1) {
                    let err_packet = ERRORPacket::IllegalTftpOperation;
                    client_socket.send(&err_packet.as_bytes());
                    return Err(format!(
                        "File transmission aborted due to an error: {}",
                        err_packet.get_error_message()
                    ));
                }

                let write_bytes = file
                    .write(data_packet.get_data())
                    .map_err(|e| format!("File transmission aborted due to an error: {}", e))?;

                let ack = ACKPacket {
                    block: data_packet.block,
                };

                client_socket
                    .send(&ack.as_bytes())
                    .map_err(|e| format!("File transmission aborted due to an error: {}", e))?;

                if write_bytes < 512 {
                    println!("Download completed!");
                    println!("File path: {}", working_dir.join(filename).display());
                    return Ok(());
                }

                last_block_number = data_packet.block;
            }
        }

        ClientAction::Write => {
            let current_dir = env::current_dir()
                .map_err(|e| format!("File transmission aborted due to an error: {}", e))?;

            let mut file = File::open(current_dir.join(&client_args.filename))
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

            // Ensure connection with the server
            client_socket
                .connect(server_addr)
                .map_err(|e| format!("Unable to etablish a connection with the server: {}", e))?;

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
                    client_socket.send(&err_packet.as_bytes());
                    return Err(format!(
                        "File transmission aborted due to an error: {}",
                        err_packet.get_error_message()
                    ));
                }
            };

            if ack_packet_for_write.block != 0 {
                let err_packet = ERRORPacket::IllegalTftpOperation;
                client_socket.send(&err_packet.as_bytes());
                return Err(format!(
                    "File transmission aborted due to an error: {}",
                    err_packet.get_error_message()
                ));
            }

            let mut current_block_number = 1;

            loop {
                let mut data_buffer = [0_u8; 512];

                let read_bytes = file
                    .read(&mut data_buffer)
                    .map_err(|e| format!("File transmission aborted due to an error: {}", e))?;

                let data_packet =
                    DATAPacket::build(current_block_number, &data_buffer[..read_bytes]).unwrap();

                client_socket
                    .send(&data_packet.as_bytes())
                    .map_err(|e| format!("File transmission aborted due to an error: {}", e))?;

                // Create a buffer to store the ACK packet
                let mut response = [0_u8; 4];

                client_socket.recv(&mut response).map_err(|e| {
                    format!("Unable to receive a ACK packet from the server: {}", e)
                })?;

                let ack_packet = match TFTPPacket::parse(&response) {
                    Ok(TFTPPacket::ACK(packet)) => packet,
                    Ok(TFTPPacket::ERROR(err_packet)) => {
                        return Err(format!(
                            "File transmission aborted due to an error: {}",
                            err_packet.get_error_message()
                        ));
                    }
                    _ => {
                        let err_packet = ERRORPacket::IllegalTftpOperation;
                        client_socket.send(&err_packet.as_bytes());
                        return Err(format!(
                            "File transmission aborted due to an error: {}",
                            err_packet.get_error_message()
                        ));
                    }
                };

                if ack_packet.block != current_block_number {
                    let err_packet = ERRORPacket::IllegalTftpOperation;
                    client_socket.send(&err_packet.as_bytes());
                    return Err(format!(
                        "File transmission aborted due to an error: {}",
                        err_packet.get_error_message(),
                    ));
                }

                if read_bytes < 512 {
                    println!("Upload completed!");
                    return Ok(());
                }

                current_block_number += 1;
            }
        }
    }

    Ok(())
}
