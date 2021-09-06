use crate::BytePacketBuffer;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum TypeField {
    Unknown(u16),
    A,
}

impl TypeField {
    pub fn to_num(&self) -> u16 {
        match *self {
            TypeField::Unknown(x) => x,
            TypeField::A => 1,
        }
    }

    pub fn from_num(num: u16) -> TypeField {
        match num {
            1 => TypeField::A,
            _ => TypeField::Unknown(num),
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.to_num().to_be_bytes().to_vec()
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum ClassField {
    Unknown(u16),
    Internet,
}

impl ClassField {
    pub fn to_num(&self) -> u16 {
        match *self {
            ClassField::Unknown(x) => x,
            ClassField::Internet => 1,
        }
    }

    pub fn from_num(num: u16) -> ClassField {
        match num {
            1 => ClassField::Internet,
            _ => ClassField::Unknown(num),
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.to_num().to_be_bytes().to_vec()
    }
}

#[derive(Clone, Debug)]
pub enum ResponseCode {
    NoError = 0,
    FormatError = 1,
    ServerFail = 2,
    NameError = 3,
    NotImplemented = 4,
    Refused = 5,
}

impl ResponseCode {
    pub fn from_num(num: u8) -> ResponseCode {
        match num {
            1 => ResponseCode::FormatError,
            2 => ResponseCode::ServerFail,
            3 => ResponseCode::NameError,
            4 => ResponseCode::NotImplemented,
            5 => ResponseCode::Refused,
            0 | _ => ResponseCode::NoError,
        }
    }

    pub fn to_num(&self) -> u8 {
        // TODO: impl for all
        match *self {
            ResponseCode::NoError => 0,
            _ => 1,
        }
    }
}

#[derive(Clone, Debug)]
pub enum OpCode {
    QUERY = 0,
    IQUERY = 1,
    STATUS = 2,
}

impl OpCode {
    pub fn from_num(num: u8) -> OpCode {
        match num {
            1 => OpCode::IQUERY,
            2 => OpCode::STATUS,
            0 | _ => OpCode::QUERY,
        }
    }

    pub fn to_num(&self) -> u8 {
        match *self {
            OpCode::QUERY => 0,
            OpCode::IQUERY => 1,
            OpCode::STATUS => 2,
        }
    }
}

pub fn read_qname(buf: &mut BytePacketBuffer) -> Result<String> {
    let mut res = "".to_string();
    let mut final_pos = 0;
    let mut exist_pointer = false;

    loop {
        let len = buf.read()? as usize;

        // len == 0x00 -> 末尾
        // len == 0xC0 -> ポインタ
        // else        -> レングスオクテット
        if len == 0x00 {
            // 最後は必ずここを通る
            // ポインタありの場合 -> 記録しておいたfinal_posにseek
            // ポインタなしの場合 -> seek不要
            if exist_pointer {
                buf.seek(final_pos - 1)?;
            }
            break;
        } else if len == 0xC0 {
            // 初めてポインタが出現した時
            // ポインタが出現したことと，最終的な遷移先を記録する
            if !exist_pointer {
                final_pos = buf.pos() + 2;
                exist_pointer = true;
            }

            let offset = ((len as u16 ^ 0xC0) << 8) + (buf.read()? as u16);
            buf.seek(offset as usize)?;
        } else {
            let str_buffer = buf.read_range(len)?;
            res.push_str(&String::from_utf8_lossy(str_buffer).to_lowercase());
            res.push_str(".");
        }
    }

    Ok(res)
}

pub fn qname_to_bytes(qname: &str) -> Vec<u8> {
    let mut res = Vec::<u8>::new();
    for s in qname.split_terminator('.') {
        res.push(s.len() as u8);
        res.append(&mut s.as_bytes().to_vec());
    }
    res.push(0);
    res
}
