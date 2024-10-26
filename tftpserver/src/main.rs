use std::{
    env,
    fs::File,
    io::Read,
    net::{SocketAddr, UdpSocket},
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

    let current_block_number = 1;

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

    server_socket.send(&data_packet.as_bytes());
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

    server_socket.send(&ack.as_bytes());
}
