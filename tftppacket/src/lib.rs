/// Represents a TFTP DATA Packet.
pub struct DataPacket {
    pub block: u16,
    pub data: [u8; 512],
}

impl DataPacket {
    pub const OPCODE: u16 = 3;

    /// Parses a byte slice into a `DataPacket`.
    ///
    /// This function takes a byte array of length 516, which includes
    /// the opcode (2 bytes), block number (2 bytes), and data (512 bytes).
    /// It extracts these components and constructs a `DataPacket` instance.
    pub fn parse(data: &[u8; 516]) -> Result<Self, String> {
        let opcode = u16::from_be_bytes(data[0..2].try_into().unwrap());
        
        if opcode != Self::OPCODE {
            return Err(String::from("Invalid TFTP data packet"));
        }

        let block = u16::from_be_bytes(data[2..4].try_into().unwrap());

        let data: [u8; 512] = data[4..].try_into().unwrap();

        Ok(Self { block, data })
    }
}

/// Represents a TFTP RRQ Packet.
pub struct RRQPacket {
    pub filename: String,
    pub mode: String,
}

impl RRQPacket {
    pub const OPCODE: u16 = 1;

    /// Constructs a RRQ packet in byte format
    /// using the specified filename and mode.
    pub fn create_rrq_packet(filename: &str, mode: &str) -> Result<Vec<u8>, String> {
        if filename.is_empty() || mode.is_empty() {
            return Err(String::from(
                "Unable to build RRQ packet, filename or mode is empty",
            ));
        }

        let mut packet: Vec<u8> = Vec::new();

        // RRQ opcode = 1
        packet.extend_from_slice(&Self::OPCODE.to_be_bytes());

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
}