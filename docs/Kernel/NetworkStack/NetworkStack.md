# Network Stack
## Overview
- Layered design isolates different network functions.
- Each layer provides services to the layer above it.

## Physical Layer
- The physical layer is responsible for transmitting raw bits over a physical medium.
- The physical layer is responsible for encoding and decoding signals.
- The physical layer is implemented in hardware.

## Data Link Layer
- The data link layer is responsible for transmitting frames over a physical medium.
- The data link layer is responsible for error detection and correction.
- The data link layer is responsible for flow control.

### MACSec
- MACSec is used to secure the data link layer over Ethernet.

### WPA3
- WPA3 is used to secure the data link layer over Wi-Fi.
- WPA2 is also supported for backward compatibility, until WPA3 becomes the standard. 

## Network Layer
- The network layer is responsible for routing packets from the source to the destination.
- Only supports IPv6.
- Extendable firewall for packet filtering.

### Protocols
- Uses IPv6 as the default protocol.
- Uses ICMPv6 for error reporting and diagnostics.
- Uses IPSec for security.

### Firewall
- The firewall is a list of simple to understand rules in a toml file.
- The rules can be adjusted, deleted, or added at runtime.
- The firewall is stateful and can track the state of connections.

### Routing
- Secure routing protocols are used in tandem with IPSec.

## Transport Layer
- The transport layer is responsible for end-to-end communication.

### TLS
- TLS is used to secure the transport layer, and is required for all TCP connections.

### QUIC
- All UDP connections use QUIC for improved performance and security.
- Secure by default, and uses TLS for encryption.

## Session Layer

## Presentation Layer

## Application Layer


