use crate::dns_message::utils;
use crate::BytePacketBuffer;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug)]
pub struct DnsQuestion {
    pub qname: String,
    pub qtype: utils::TypeField,
    pub qclass: utils::ClassField,
}

impl DnsQuestion {
    pub fn new() -> DnsQuestion {
        DnsQuestion {
            qname: "".to_string(),
            qtype: utils::TypeField::A,
            qclass: utils::ClassField::Internet,
        }
    }

    pub fn from_buf(buf: &mut BytePacketBuffer) -> Result<DnsQuestion> {
        let mut res = DnsQuestion::new();
        res.read(buf)?;

        return Ok(res);
    }

    pub fn read(&mut self, buf: &mut BytePacketBuffer) -> Result<()> {
        self.qname = utils::read_qname(buf)?;
        self.qtype = utils::TypeField::from_num(buf.read_u16()?);
        self.qclass = utils::ClassField::from_num(buf.read_u16()?);

        Ok(())
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut res = Vec::new();
        res.append(&mut utils::qname_to_bytes(&self.qname));
        res.append(&mut self.qtype.as_bytes());
        res.append(&mut self.qclass.as_bytes());
        res
    }
}
