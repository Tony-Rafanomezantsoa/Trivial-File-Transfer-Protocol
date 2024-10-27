use std::{
    env,
    fs::{File, OpenOptions},
    io::{Read, Write},
    net::{SocketAddr, UdpSocket},
    path::Path,
};

use rand::Rng;
use tftppacket::{ACKPacket, DATAPacket, ERRORPacket, RRQPacket, TFTPPacket, WRQPacket};

fn main() -> Result<(), String> {
    let server_socket = UdpSocket::bind("0.0.0.0:69")
        .map_err(|e| format!("Unable to initialize a UDP server socket: {}", e))?;

    println!("The TFTP server is running successfully...");

    loop {
        // Create a buffer to store a TFTP request
        let mut request = [0_u8; 512];

        let (req_bytes, client_addr) = match server_socket.recv_from(&mut request) {
            Ok(recv_info) => recv_info,
            Err(e) => {
                eprintln!("Unable to receive a TFTP request packet: {}", e);
                continue;
            }
        };

        match TFTPPacket::parse(&request[..req_bytes]) {
            Ok(TFTPPacket::RRQ(rrq)) => client_read_from_server(rrq, client_addr),
            Ok(TFTPPacket::WRQ(wrq)) => client_write_to_server(wrq, client_addr),
            _ => {
                let err_packet = ERRORPacket::IllegalTftpOperation;
                server_socket.send_to(&err_packet.as_bytes(), client_addr);
                eprintln!("Error: {}", err_packet.get_error_message());
                continue;
            }
        }
    }
}

fn client_read_from_server(rrq: RRQPacket, client_addr: SocketAddr) {
    let server_socket = loop {
        let server_tid: u16 = rand::thread_rng().gen_range(0..65535);

        match UdpSocket::bind(format!("0.0.0.0:{}", server_tid)) {
            Ok(socket) => break socket,
            Err(_) => continue,
        }
    };

    // Ensure connection with the client
    if let Err(e) = server_socket.connect(client_addr) {
        eprintln!("Error: {}", e);
        return;
    }

    if rrq.mode.to_lowercase() != "octet" {
        let err_packet =
            ERRORPacket::NotDefined("The server supports only the 'octet' mode".to_string());
        server_socket.send(&err_packet.as_bytes());
        eprintln!("Error: {}", err_packet.get_error_message());
        return;
    }

    let bin_dir = match env::current_exe() {
        Ok(bin_path) => bin_path.parent().unwrap().to_owned(),
        Err(e) => {
            let err_packet =
                ERRORPacket::NotDefined(format!("An error occurs on the server: {}", e));
            server_socket.send(&err_packet.as_bytes());
            eprintln!("Error: {}", e);
            return;
        }
    };

    let mut file = match File::open(bin_dir.join(&rrq.filename)) {
        Ok(f) => f,
        Err(e) => {
            let err_packet =
                ERRORPacket::NotDefined(format!("An error occurs on the server: {}", e));
            server_socket.send(&err_packet.as_bytes());
            eprintln!("Error: {}", e);
            return;
        }
    };

    let mut current_block_number = 1;

    loop {
        let mut data_buffer = [0_u8; 512];

        let read_bytes = match file.read(&mut data_buffer) {
            Ok(bytes) => bytes,
            Err(e) => {
                let err_packet =
                    ERRORPacket::NotDefined(format!("An error occurs on the server: {}", e));
                server_socket.send(&err_packet.as_bytes());
                eprintln!("Error: {}", e);
                return;
            }
        };

        let data_packet =
            DATAPacket::build(current_block_number, &data_buffer[..read_bytes]).unwrap();

        if let Err(e) = server_socket.send(&data_packet.as_bytes()) {
            eprintln!("Error: {}", e);
            return;
        }

        // Creates buffer to store
        // the ACK packet
        let mut response = [0_u8; 4];

        if let Err(e) = server_socket.recv(&mut response) {
            eprintln!("Error: {}", e);
            return;
        }

        let ack = match TFTPPacket::parse(&response) {
            Ok(TFTPPacket::ACK(packet)) => packet,
            _ => {
                let err_packet = ERRORPacket::IllegalTftpOperation;
                server_socket.send(&err_packet.as_bytes());
                eprintln!("Error: {}", err_packet.get_error_message());
                return;
            }
        };

        if ack.block != current_block_number {
            let err_packet = ERRORPacket::IllegalTftpOperation;
            server_socket.send(&err_packet.as_bytes());
            eprintln!("Error: {}", err_packet.get_error_message());
            return;
        }

        if read_bytes < 512 {
            println!("File transmission is finished [DOWNLOAD]");
            return;
        }

        current_block_number += 1;
    }
}

fn client_write_to_server(wrq: WRQPacket, client_addr: SocketAddr) {
    let server_socket = loop {
        let server_tid: u16 = rand::thread_rng().gen_range(0..65535);

        match UdpSocket::bind(format!("0.0.0.0:{}", server_tid)) {
            Ok(socket) => break socket,
            Err(_) => continue,
        }
    };

    // Ensure connection with the client
    if let Err(e) = server_socket.connect(client_addr) {
        eprintln!("Error: {}", e);
        return;
    }

    if wrq.mode.to_lowercase() != "octet" {
        let err_packet =
            ERRORPacket::NotDefined("The server supports only the 'octet' mode".to_string());
        server_socket.send(&err_packet.as_bytes());
        eprintln!("Error: {}", err_packet.get_error_message());
        return;
    }

    let ack = ACKPacket { block: 0 };

    if let Err(e) = server_socket.send(&ack.as_bytes()) {
        eprint!("Error: {}", e);
        return;
    }

    let bin_dir = match env::current_exe() {
        Ok(bin_path) => bin_path.parent().unwrap().to_owned(),
        Err(e) => {
            let err_packet =
                ERRORPacket::NotDefined(format!("An error occurs on the server: {}", e));
            server_socket.send(&err_packet.as_bytes());
            eprintln!("Error: {}", e);
            return;
        }
    };

    let filename = match Path::new(&wrq.filename).file_name() {
        Some(filename) => filename,
        None => {
            let err_packet = ERRORPacket::NotDefined("Invalid Filename".to_string());
            server_socket.send(&err_packet.as_bytes());
            eprintln!("Error: Invalid filename from client [UPLOAD]");
            return;
        }
    };

    let mut file = match OpenOptions::new()
        .create_new(true)
        .append(true)
        .open(bin_dir.join(filename))
    {
        Ok(f) => f,
        Err(e) => {
            let err_packet =
                ERRORPacket::NotDefined(format!("An error occurs on the server: {}", e));
            server_socket.send(&err_packet.as_bytes());
            eprintln!("Error: {}", e);
            return;
        }
    };

    let mut last_block_number = 0;

    loop {
        // Create a buffer to store
        // a TFTP DATA packet (516 bytes)
        let mut data_buffer = [0_u8; 516];

        let read_bytes = match server_socket.recv(&mut data_buffer) {
            Ok(bytes) => bytes,
            Err(e) => {
                eprintln!("Error: {}", e);
                return;
            }
        };

        let data_packet = match TFTPPacket::parse(&data_buffer[..read_bytes]) {
            Ok(TFTPPacket::DATA(packet)) => packet,
            Ok(TFTPPacket::ERROR(err_packet)) => {
                eprintln!("Error: {}", err_packet.get_error_message());
                return;
            }
            _ => {
                let err_packet = ERRORPacket::IllegalTftpOperation;
                server_socket.send(&err_packet.as_bytes());
                eprintln!("Error: {}", err_packet.get_error_message());
                return;
            }
        };

        if data_packet.block != (last_block_number + 1) {
            let err_packet = ERRORPacket::IllegalTftpOperation;
            server_socket.send(&err_packet.as_bytes());
            eprintln!("Error: {}", err_packet.get_error_message());
            return;
        }

        let write_bytes = match file.write(data_packet.get_data()) {
            Ok(bytes) => bytes,
            Err(e) => {
                let err_packet =
                    ERRORPacket::NotDefined(format!("An error occurs on the server: {}", e));
                server_socket.send(&err_packet.as_bytes());
                eprintln!("Error: {}", e);
                return;
            }
        };

        let ack = ACKPacket {
            block: data_packet.block,
        };

        if let Err(e) = server_socket.send(&ack.as_bytes()) {
            eprintln!("Error: {}", e);
            return;
        }

        if write_bytes < 512 {
            println!("File transmission is finished [UPLOAD]");
            return;
        }

        last_block_number = data_packet.block;
    }
}
