use std::collections::HashMap;
use std::net::{SocketAddr, UdpSocket};
use std::str::FromStr;

mod byte_packet_buffer;
mod dns_message;
use crate::byte_packet_buffer::BytePacketBuffer;
use crate::dns_message::DnsMessage;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

fn handle_query(
    socket: &UdpSocket,
    destination_table: &mut HashMap<u16, SocketAddr>,
) -> Result<()> {
    let mut buf = BytePacketBuffer::new();
    let (_, from_addr) = socket.recv_from(&mut buf.buf)?;
    let dns_message = DnsMessage::from_buf(&mut buf)?;

    let destination_address = if dns_message.header.is_response {
        destination_table[&dns_message.header.id]
    } else {
        destination_table.insert(dns_message.header.id, from_addr);
        SocketAddr::from_str("8.8.8.8:53").unwrap()
    };

    socket
        .send_to(dns_message.as_bytes().as_slice(), destination_address)
        .unwrap();

    Ok(())
}

fn main() -> Result<()> {
    let socket = UdpSocket::bind(("0.0.0.0", 2053))?;

    let mut destination_table: HashMap<u16, SocketAddr> = HashMap::new();

    loop {
        match handle_query(&socket, &mut destination_table) {
            Ok(_) => {}
            Err(e) => eprintln!("An error occurred: {}", e),
        }
    }
}
