mod dns_header;
mod dns_question;
mod dns_record;
mod utils;
use dns_header::DnsHeader;
use dns_question::DnsQuestion;
use dns_record::DnsRecord;

use crate::byte_packet_buffer::BytePacketBuffer;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

pub struct DnsMessage {
    pub header: DnsHeader,
    pub questions: Vec<DnsQuestion>,
    pub answers: Vec<DnsRecord>,
    pub authorities: Vec<DnsRecord>,
    pub additionals: Vec<DnsRecord>,
}

impl DnsMessage {
    pub fn new() -> DnsMessage {
        DnsMessage {
            header: DnsHeader::new(),
            questions: Vec::new(),
            answers: Vec::new(),
            authorities: Vec::new(),
            additionals: Vec::new(),
        }
    }

    pub fn from_buf(buf: &mut BytePacketBuffer) -> Result<DnsMessage> {
        let mut res = DnsMessage::new();
        res.header.read(buf)?;

        // read questions
        for _ in 0..res.header.q_count {
            res.questions.push(DnsQuestion::from_buf(buf)?)
        }
        // read answers
        for _ in 0..res.header.an_count {
            res.answers.push(DnsRecord::from_buf(buf)?)
        }
        // read authorities
        for _ in 0..res.header.ns_count {
            res.authorities.push(DnsRecord::from_buf(buf)?)
        }
        // read additionals
        for _ in 0..res.header.ar_count {
            res.additionals.push(DnsRecord::from_buf(buf)?)
        }

        Ok(res)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut res = Vec::new();
        res.append(&mut self.header.as_bytes());
        for question in self.questions.iter() {
            res.append(&mut question.as_bytes());
        }
        for answer in self.answers.iter() {
            res.append(&mut answer.as_bytes());
        }
        res
    }
}
