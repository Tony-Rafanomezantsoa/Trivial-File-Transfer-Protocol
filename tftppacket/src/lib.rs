/// Represents a TFTP RRQ Packet.
#[derive(Debug)]
pub struct RRQPacket {
    pub filename: String,
    pub mode: String,
}

impl RRQPacket {
    pub const OPCODE: u16 = 1;

    /// Constructs a TFTP RRQ packet in byte format
    /// using the specified filename and mode.
    pub fn create_rrq_packet(filename: &str, mode: &str) -> Vec<u8> {
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

        packet
    }

    /// Parses a raw byte slice into a `RRQPacket`
    pub fn parse(data: &[u8]) -> Result<Self, String> {
        let opcode = match data.get(0..2) {
            Some(v) => u16::from_be_bytes(v.try_into().unwrap()),
            None => return Err(String::from("Invalid RRQ packet")),
        };

        if opcode != Self::OPCODE {
            return Err(String::from("Invalid RRQ packet"));
        }

        let mut data_iterator = data
            .iter()
            .enumerate()
            .skip_while(|(i, _)| *i == 0 || *i == 1); // Skip opcode bytes

        let mut filename_bytes: Vec<u8> = Vec::new();

        // Obtain the filename in byte format
        loop {
            match data_iterator.next() {
                Some((_, byte)) => {
                    if *byte == 0 {
                        break;
                    }
                    filename_bytes.push(*byte);
                }
                None => return Err(String::from("Invalid RRQ packet")),
            }
        }

        let mut mode_bytes: Vec<u8> = Vec::new();

        // Obtain the mode in byte format
        loop {
            match data_iterator.next() {
                Some((_, byte)) => {
                    if *byte == 0 {
                        break;
                    }
                    mode_bytes.push(*byte);
                }
                None => return Err(String::from("Invalid RRQ packet")),
            }
        }

        let filename = String::from_utf8(filename_bytes).map_err(|_| String::from("Invalid RRQ packet"))?;
        let mode = String::from_utf8(mode_bytes).map_err(|_| String::from("Invalid RRQ packet"))?;

        Ok(Self { filename, mode })
    }
}

/// Represents a TFTP WRQ Packet.
pub struct WRQPacket {
    pub filename: String,
    pub mode: String,
}

impl WRQPacket {
    pub const OPCODE: u16 = 2;

    /// Constructs a TFTP WRQ packet in byte format
    /// using the specified filename and mode.
    pub fn create_wrq_packet(filename: &str, mode: &str) -> Vec<u8> {
        let mut packet: Vec<u8> = Vec::new();

        // WRQ opcode = 2
        packet.extend_from_slice(&Self::OPCODE.to_be_bytes());

        // WRQ filename
        packet.extend_from_slice(filename.as_bytes());

        // WRQ null byte
        packet.push(0);

        // WRQ mode
        packet.extend_from_slice(mode.as_bytes());

        // WRQ null byte
        packet.push(0);

        packet
    }
}

/// Represents a TFTP DATA Packet.
pub struct DATAPacket {
    pub block: u16,
    pub data: [u8; 512],
}

impl DATAPacket {
    pub const OPCODE: u16 = 3;

    /// Parses a byte array into a `DATAPacket`.
    ///
    /// This function takes a byte array of length 516, which includes
    /// the opcode (2 bytes), block number (2 bytes), and data (512 bytes).
    /// It extracts these components and constructs a `DATAPacket` instance.
    pub fn parse(data: [u8; 516]) -> Result<Self, String> {
        let opcode = u16::from_be_bytes(data[0..2].try_into().unwrap());

        if opcode != Self::OPCODE {
            return Err(String::from("Invalid TFTP DATA packet"));
        }

        let block = u16::from_be_bytes(data[2..4].try_into().unwrap());

        let data: [u8; 512] = data[4..].try_into().unwrap();

        Ok(Self { block, data })
    }
}

/// Represents a TFTP ACK Packet.
pub struct ACKPacket {
    pub block: u16,
}

impl ACKPacket {
    pub const OPCODE: u16 = 4;

    /// Parses a byte array into a `ACKPacket`.
    ///
    /// This function takes a byte array of length 4, which includes
    /// the opcode (2 bytes) and block number (2 bytes).
    /// It extracts these components and constructs a `ACKPacket` instance.
    pub fn parse(data: [u8; 4]) -> Result<Self, String> {
        let opcode = u16::from_be_bytes(data[0..2].try_into().unwrap());

        if opcode != Self::OPCODE {
            return Err(String::from("Invalid TFTP ACK packet"));
        }

        let block = u16::from_be_bytes(data[2..4].try_into().unwrap());

        Ok(Self { block })
    }
}

/// Represents a TFTP ERROR Packet.
pub enum ERRORPacket {}

impl ERRORPacket {
    pub const OPCODE: u16 = 5;

    /// Constructs a custom TFTP ERROR packet in byte format
    /// with ErrorCode = 0 and using the specified error message.
    pub fn create_custom_error_packet(error_message: &str) -> Vec<u8> {
        let mut packet: Vec<u8> = Vec::new();

        // ERROR opcode = 5
        packet.extend_from_slice(&Self::OPCODE.to_be_bytes());

        // Custom ErrCode = 0
        packet.extend_from_slice(&[0, 0]);

        // ErrorMessage
        packet.extend_from_slice(error_message.as_bytes());

        // ERROR null byte
        packet.push(0);

        packet
    }
}
