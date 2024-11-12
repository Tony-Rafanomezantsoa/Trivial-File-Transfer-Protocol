# TFTP Implementation (RFC 1350)

This project is a **C** implementation of the **[Trivial File Transfer Protocol (TFTP)](https://tools.ietf.org/html/rfc1350)**, as defined by **RFC 1350**. TFTP is a simple, lightweight protocol designed for transferring files over UDP, typically used in environments with limited resources such as bootstrapping networked devices or embedded systems.

## Features

- **Client-Server Architecture**: Implements both the **TFTP client** and **TFTP server** components.
- **UDP-based Communication**: All data is transferred over **UDP**, as specified in RFC 1350, for low-latency and connectionless operation.
- **Read and Write Operations**: Supports **Read Request (RRQ)** and **Write Request (WRQ)** operations for file transfer.
- **Block-based Data Transfer**: Data is transmitted in fixed-size blocks, and acknowledgements are sent for each block.
- **Error Handling**: Implements the standard TFTP error codes (e.g., `0` for "Not defined", `1` for "File not found", `2` for "Access violation", etc.) for error reporting.

## Key Protocol Details

1. **Operations**: 
   - **RRQ (Read Request)**: A client sends an RRQ to request a file from the server.
   - **WRQ (Write Request)**: A client sends a WRQ to upload a file to the server.
   
2. **Data Transfer**:
   - Files are transferred in **512-byte blocks**, with the server sending blocks of data and the client acknowledging each block.
   
3. **Error Codes**:
   - The protocol defines several error codes that allow clients and servers to report specific issues such as "File not found", "Access violation", "Disk full", etc.
   
4. **Timeouts**:
   - The client and server use timeout and retransmission mechanisms to ensure reliability in the connectionless UDP environment.

## Overview

- The **TFTP server** listens on a specific port (typically UDP port 69) for incoming requests. Once a request is received, it either transfers the requested file or generates an error response.
- The **TFTP client** initiates file transfers by sending an RRQ or WRQ to the server, waits for the server's response, and handles data block transfers in a loop.

The protocol is minimalistic and does not support advanced features like authentication, encryption, or extensive error recovery. It is designed to be lightweight and fast for specific use cases.

## Example Workflow

### Client to Server:

1. **Client sends RRQ** (Read Request) for a file:
   - Client sends a UDP packet to the server with the file name and request type.
   
2. **Server responds with Data Blocks**:
   - The server sends the file in **512-byte data blocks**.
   
3. **Client Acknowledges**:
   - After receiving each block, the client sends an acknowledgement (ACK) for the block.
   
4. **Transfer Complete**:
   - The server sends a final block with fewer than 512 bytes, signaling the end of the transfer.

### Server to Client:

1. **Client sends WRQ** (Write Request) to upload a file:
   - Client initiates a write request with the file name.
   
2. **Server Sends ACK**:
   - The server responds with an acknowledgement indicating that it is ready to receive data.
   
3. **Client Sends Data Blocks**:
   - The client sends the file in **512-byte blocks**, and the server acknowledges each block with an ACK.
   
4. **Transfer Complete**:
   - The client sends a final block with fewer than 512 bytes, and the server sends the last ACK, completing the transfer.

## Error Handling

- **Error Codes**: Errors such as file not found, access violations, and out-of-space conditions are communicated via standard TFTP error codes.
- If an error occurs (e.g., trying to read a non-existent file), the server sends an **ERROR** packet containing the error code and message.

---

This implementation focuses on the core aspects of TFTP as defined in RFC 1350. It does not include any advanced features beyond what is required for file transfer using the basic protocol.

