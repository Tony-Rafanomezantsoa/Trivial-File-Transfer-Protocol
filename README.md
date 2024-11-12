# TFTP Implementation (RFC 1350)

This project is an implementation of the **[Trivial File Transfer Protocol (TFTP)](https://tools.ietf.org/html/rfc1350)**, as defined by **RFC 1350**. TFTP is a simple, lightweight protocol designed for transferring files over UDP, typically used in environments with limited resources such as bootstrapping networked devices or embedded systems.

## Features

- **Client-Server Architecture**: Implements both the **TFTP client** and **TFTP server** components.
- **UDP-based Communication**: All data is transferred over **UDP**, as specified in RFC 1350, for low-latency and connectionless operation.
- **Read and Write Operations**: Supports **Read Request (RRQ)** and **Write Request (WRQ)** operations for file transfer.
- **Block-based Data Transfer**: Data is transmitted in fixed-size blocks, and acknowledgements are sent for each block.
- **Error Handling**: Implements the standard TFTP error codes (e.g., `0` for "Not defined", `1` for "File not found", `2` for "Access violation", etc.) for error reporting.

---

This implementation focuses on the core aspects of TFTP as defined in RFC 1350. It does not include any advanced features beyond what is required for file transfer using the basic protocol.

