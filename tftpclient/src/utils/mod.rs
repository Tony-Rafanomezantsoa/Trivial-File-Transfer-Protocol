use std::{env, error::Error, net::IpAddr};

/// Represents the actions that a TFTP client can perform.
///
/// This enum defines the two primary operations for a TFTP client:
/// reading from and writing to a remote server.
#[derive(Debug)]
pub enum ClientAction {
    Read,
    Write,
}

/// Contains all arguments required by a TFTP client.
///
/// This struct holds the necessary parameters for performing TFTP operations,
/// including the action to be taken, the target filename, the remote IP address
/// of the server, and the mode of transfer.
#[derive(Debug)]
pub struct ClientArgs {
    pub action: ClientAction,
    pub filename: String,
    pub remote_ip: IpAddr,
    pub mode: String,
}

impl ClientArgs {
    /// Constructs a new instance of `ClientArgs`.
    pub fn build() -> Result<Self, String> {
        let args = env::args().collect::<Vec<String>>();

        if args.len() != 5 {
            return Err(String::from("Invalid arguments"));
        }

        let action;

        match args[1].to_lowercase().as_str() {
            "write" => action = ClientAction::Write,
            "read" => action = ClientAction::Read,
            _ => return Err(String::from("Invalid [ACTION]")),
        }

        if args[2].is_empty() {
            return Err(String::from("[FILENAME] is empty"));
        }

        let remote_ip = match args[3].parse::<IpAddr>() {
            Ok(ip) => ip,
            Err(_) => return Err(String::from("Invalid [REMOTE IP ADDRESS]")),
        };

        if args[4].is_empty() {
            return Err(String::from("[MODE] is empty"));
        }

        Ok(Self {
            action,
            filename: args[2].clone(),
            remote_ip,
            mode: args[4].clone(),
        })
    }
}

/// Constructs a Read Request (RRQ) packet for the
/// Trivial File Transfer Protocol (TFTP) in byte format.
pub fn build_read_request_packet(filename: &str, mode: &str) -> Result<Vec<u8>, String> {
    if filename.is_empty() || mode.is_empty() {
        return Err(String::from(
            "Unable to build RRQ packet, filename or mode is empty",
        ));
    }

    let mut packet: Vec<u8> = Vec::new();

    // RRQ opcode = 1
    packet.extend_from_slice(&1_u16.to_be_bytes());

    // RRQ filename
    packet.extend_from_slice(filename.as_bytes());

    // RRQ null byte
    packet.push(0);

    // RRQ mode
    packet.extend_from_slice(mode.as_bytes());

    // RRQ null byte
    packet.push(0);

    Ok(packet)
}

/// Represents a TFTP Data Packet.
pub struct DataPacket {
    pub opcode: u16,
    pub block: u16,
    pub data: [u8; 512],
}

impl DataPacket {
    /// Parses a byte slice into a `DataPacket`.
    ///
    /// This function takes a byte array of length 516, which includes
    /// the opcode (2 bytes), block number (2 bytes), and data (512 bytes).
    /// It extracts these components and constructs a `DataPacket` instance.
    pub fn parse(data: &[u8; 516]) -> Result<Self, String> {
        let opcode = u16::from_be_bytes(data[0..2].try_into().unwrap());
        
        if opcode != 3 {
            return Err(String::from("Invalid TFTP data packet"));
        }

        let block = u16::from_be_bytes(data[2..4].try_into().unwrap());

        let data: [u8; 512] = data[4..].try_into().unwrap();

        Ok(Self { opcode, block, data })
    }
}
