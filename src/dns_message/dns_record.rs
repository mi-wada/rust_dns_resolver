use std::net::Ipv4Addr;

use crate::dns_message::utils;
use crate::BytePacketBuffer;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug)]
pub struct DnsRecord {
    pub name: String,
    pub rtype: utils::TypeField,
    pub rclass: utils::ClassField,
    pub ttl: u32,
    pub rdlength: u16,
    pub rdata: DnsRecordData,
}

#[derive(Clone, Debug)]
pub enum DnsRecordData {
    A { addr: Ipv4Addr },
    Unknown,
}

impl DnsRecordData {
    pub fn as_bytes(&self) -> Vec<u8> {
        match *self {
            DnsRecordData::A { addr } => addr.octets().to_vec(),
            DnsRecordData::Unknown => vec![0],
        }
    }
}

impl DnsRecord {
    pub fn new() -> DnsRecord {
        DnsRecord {
            name: "".to_string(),
            rtype: utils::TypeField::A,
            rclass: utils::ClassField::Internet,
            ttl: 0,
            rdlength: 0,
            rdata: DnsRecordData::Unknown,
        }
    }

    pub fn from_buf(buf: &mut BytePacketBuffer) -> Result<DnsRecord> {
        let mut res = DnsRecord::new();
        res.read(buf)?;

        Ok(res)
    }

    pub fn read(&mut self, buf: &mut BytePacketBuffer) -> Result<()> {
        self.name = utils::read_qname(buf)?;
        self.rtype = utils::TypeField::from_num(buf.read_u16()?);
        self.rclass = utils::ClassField::from_num(buf.read_u16()?);
        self.ttl = buf.read_u32()?;
        self.rdlength = buf.read_u16()?;

        if self.rtype == utils::TypeField::A && self.rclass == utils::ClassField::Internet {
            let raw_addr = buf.read_u32()?;
            let addr = Ipv4Addr::new(
                ((raw_addr >> 24) & 0xFF) as u8,
                ((raw_addr >> 16) & 0xFF) as u8,
                ((raw_addr >> 8) & 0xFF) as u8,
                ((raw_addr >> 0) & 0xFF) as u8,
            );
            self.rdata = DnsRecordData::A { addr };
        } else {
            self.rdata = DnsRecordData::Unknown;
        }

        Ok(())
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut res = Vec::<u8>::new();
        res.append(&mut utils::qname_to_bytes(&self.name));
        res.append(&mut self.rtype.as_bytes());
        res.append(&mut self.rclass.as_bytes());
        res.append(&mut self.ttl.to_be_bytes().to_vec());
        res.append(&mut self.rdlength.to_be_bytes().to_vec());
        res.append(&mut self.rdata.as_bytes());
        res
    }
}
