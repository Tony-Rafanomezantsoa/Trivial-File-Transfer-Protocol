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
#[derive(Debug)]
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

    /// Parses a raw byte slice into a `WRQPacket`
    pub fn parse(data: &[u8]) -> Result<Self, String> {
        let opcode = match data.get(0..2) {
            Some(v) => u16::from_be_bytes(v.try_into().unwrap()),
            None => return Err(String::from("Invalid WRQ packet")),
        };

        if opcode != Self::OPCODE {
            return Err(String::from("Invalid WRQ packet"));
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
                None => return Err(String::from("Invalid WRQ packet")),
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
                None => return Err(String::from("Invalid WRQ packet")),
            }
        }

        let filename = String::from_utf8(filename_bytes).map_err(|_| String::from("Invalid WRQ packet"))?;
        let mode = String::from_utf8(mode_bytes).map_err(|_| String::from("Invalid WRQ packet"))?;

        Ok(Self { filename, mode })
    }
}

/// Represents a TFTP DATA Packet.
#[derive(Debug)]
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
#[derive(Debug)]
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
#[derive(Debug)]
pub enum ERRORPacket {
    NotDefined(String),
    FileNotFound,
    AccessViolation,
    DiskFull,
    IllegalTftpOperation,
    UknownTransferID,
    FileAlreadyExists,
    NoSuchUser,
}

impl ERRORPacket {
    pub const OPCODE: u16 = 5;

    /// Constructs a custom TFTP ERROR packet in byte format
    /// using the specified error code and error message.
    fn create_custom_error_packet(error_code: u16, error_message: &str) -> Vec<u8> {
        let mut packet: Vec<u8> = Vec::new();

        // ERROR opcode = 5
        packet.extend_from_slice(&Self::OPCODE.to_be_bytes());

        // Custom ErrCode
        packet.extend_from_slice(&error_code.to_be_bytes());

        // ErrorMessage
        packet.extend_from_slice(error_message.as_bytes());

        // ERROR null byte
        packet.push(0);

        packet
    }

    /// Converts a `ERRORPacket` into a TFTP ERROR packet in byte format.
    pub fn as_bytes(&self) -> Vec<u8> {
        match *self {
            Self::NotDefined(ref error_message) => Self::create_custom_error_packet(0, error_message),
            Self::FileNotFound => Self::create_custom_error_packet(1, "File not found."),
            Self::AccessViolation => Self::create_custom_error_packet(2, "Access violation."),
            Self::DiskFull => Self::create_custom_error_packet(3, "Disk full or allocation exceeded."),
            Self::IllegalTftpOperation => Self::create_custom_error_packet(4, "Illegal TFTP operation."),
            Self::UknownTransferID => Self::create_custom_error_packet(5, "Unknown transfer ID."),
            Self::FileAlreadyExists => Self::create_custom_error_packet(6, "File already exists."),
            Self::NoSuchUser => Self::create_custom_error_packet(7, "No such user."),
        }
    }

    /// Parses a byte array into a `ERRORPacket`.
    pub fn parse(data: &[u8]) -> Result<Self, String> {
        let opcode = match data.get(0..2) {
            Some(v) => u16::from_be_bytes(v.try_into().unwrap()),
            None => return Err(String::from("Invalid ERROR packet")),
        };

        if opcode != Self::OPCODE {
            return Err(String::from("Invalid ERROR packet"));
        }

        let errcode = match data.get(2..4) {
            Some(v) => u16::from_be_bytes(v.try_into().unwrap()),
            None => return Err(String::from("Invalid ERROR packet")),
        };

        match errcode {
            0 => {
                let mut error_message_bytes: Vec<u8> = Vec::new();

                for (i, byte) in data.iter().enumerate() {
                    // Skip Opcode and ErrCode
                    if i < 4 {
                        continue;
                    }

                    if *byte == 0 {
                        break;
                    }

                    error_message_bytes.push(*byte);
                }

                match String::from_utf8(error_message_bytes) {
                    Ok(error_message) => Ok(Self::NotDefined(error_message)),
                    Err(_) => Err(String::from("Invalid ERROR packet")),
                }
            }
            1 => Ok(Self::FileNotFound),
            2 => Ok(Self::AccessViolation),
            3 => Ok(Self::DiskFull),
            4 => Ok(Self::IllegalTftpOperation),
            5 => Ok(Self::UknownTransferID),
            6 => Ok(Self::FileAlreadyExists),
            7 => Ok(Self::NoSuchUser),
            _ => Err(String::from("Invalid ERROR packet")),
        }
    }
}

/// Represents all TFTP packets
/// decsribed in RFC 1350.
pub enum TFTPPacket {
    RRQ(RRQPacket),
    WRQ(WRQPacket),
    DATA(DATAPacket),
    ACK(ACKPacket),
    ERROR(ERRORPacket),
}

impl TFTPPacket {
    /// Parses a raw byte slice into a `RequestPacket`,
    /// which can be either a Read Request (RRQ) 
    /// or a Write Request (WRQ) packet.
    pub fn parse(data: &[u8]) -> Result<Self, String> {
        if let Ok(rrq) = RRQPacket::parse(data) {
            return Ok(Self::RRQ(rrq));
        }

        if let Ok(wrq) = WRQPacket::parse(data) {
            return Ok(Self::WRQ(wrq));
        }

        Err(String::from("Invalid TFTP Request Packet"))
    }
}