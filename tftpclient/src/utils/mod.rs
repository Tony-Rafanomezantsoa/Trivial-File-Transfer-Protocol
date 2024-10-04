use std::{env, net::IpAddr};

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
            return Err(String::from("Error: Invalid arguments"));
        }

        let action;

        match args[1].to_lowercase().as_str() {
            "write" => action = ClientAction::Write,
            "read" => action = ClientAction::Read,
            _ => return Err(String::from("Error: Invalid [ACTION]")),
        }

        if args[2].is_empty() {
            return Err(String::from("Error: [FILENAME] is empty"));
        }

        let remote_ip = match args[3].parse::<IpAddr>() {
            Ok(ip) => ip,
            Err(_) => return Err(String::from("Error: Invalid [REMOTE IP ADDRESS]")),
        };

        if args[4].is_empty() {
            return Err(String::from("Error: [MODE] is empty"));
        }
        
        Ok(Self {
            action,
            filename: args[2].clone(),
            remote_ip,
            mode: args[4].clone(),
        })
    }
}