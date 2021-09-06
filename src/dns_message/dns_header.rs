use crate::dns_message::utils;
use crate::BytePacketBuffer;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug)]
pub struct DnsHeader {
    pub id: u16,
    pub is_response: bool,
    pub opcode: utils::OpCode,
    pub is_authoritative: bool,
    pub is_truncated: bool,
    pub is_recursion_desired: bool,
    pub is_recursion_available: bool,
    pub z: u8, // TODO: boolではない u4
    pub response_code: utils::ResponseCode,
    pub q_count: u16,
    pub an_count: u16,
    pub ns_count: u16,
    pub ar_count: u16,
}

impl DnsHeader {
    pub fn new() -> DnsHeader {
        DnsHeader {
            id: 0,
            is_response: false,
            opcode: utils::OpCode::QUERY,
            is_authoritative: false,
            is_truncated: false,
            is_recursion_desired: false,
            is_recursion_available: false,
            z: 0,
            response_code: utils::ResponseCode::NoError,
            q_count: 0,
            an_count: 0,
            ns_count: 0,
            ar_count: 0,
        }
    }

    pub fn read(&mut self, buf: &mut BytePacketBuffer) -> Result<()> {
        self.id = buf.read_u16()?;
        let flags = buf.read()?;
        self.is_response = (flags & (1 << 7)) > 0;
        self.opcode = utils::OpCode::from_num(flags & (0x0F << 3));
        self.is_authoritative = (flags & (1 << 2)) > 0;
        self.is_truncated = (flags & (1 << 1)) > 0;
        self.is_recursion_desired = (flags & 1) > 0;
        let flags = buf.read()?;
        self.is_recursion_available = (flags & (1 << 7)) > 0;
        self.z = ((flags & (1 << 6)) > 0) as u8; // TODO: 予約枠だが0x010がセットされている...
        self.response_code = utils::ResponseCode::from_num(flags & 0x0F);
        self.q_count = buf.read_u16()?;
        self.an_count = buf.read_u16()?;
        self.ns_count = buf.read_u16()?;
        self.ar_count = buf.read_u16()?;

        Ok(())
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut res = Vec::new();
        res.append(&mut self.id.to_be_bytes().to_vec());
        res.push(
            ((self.is_response as u8) << 7)
                + (self.opcode.to_num() << 3)
                + ((self.is_authoritative as u8) << 2)
                + ((self.is_truncated as u8) << 1)
                + (self.is_recursion_desired as u8),
        );
        res.push(
            ((self.is_recursion_available as u8) << 7)
                + (self.z << 4)
                + self.response_code.to_num(),
        );
        res.append(&mut self.q_count.to_be_bytes().to_vec());
        res.append(&mut self.an_count.to_be_bytes().to_vec());
        res.append(&mut self.ns_count.to_be_bytes().to_vec());
        res.append(&mut self.ar_count.to_be_bytes().to_vec());
        res
    }
}
