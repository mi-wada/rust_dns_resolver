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
        for _ in 0..res.header.question_count {
            res.questions.push(DnsQuestion::from_buf(buf)?)
        }
        for _ in 0..res.header.answer_count {
            res.answers.push(DnsRecord::from_buf(buf)?)
        }
        for _ in 0..res.header.authority_count {
            res.authorities.push(DnsRecord::from_buf(buf)?)
        }
        for _ in 0..res.header.additional_count {
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
        // TODO: impl authorities.as_bytes & additional_count.as_bytes
        res
    }
}
