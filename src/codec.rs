use crate::storage::{HEADER_LEN, TRAILER_LEN};
use crate::{Content, Error, Options, Properties, Value};

const FORMAT_DEPTH: u32 = 128;

pub(crate) fn pack(content: &Content, options: Options) -> Result<Vec<u8>, Error> {
    let mut e = Encoder {
        bytes: Vec::new(),
        limit: options.max_content_bytes,
        max_depth: options.max_depth,
    };
    e.bytes
        .try_reserve(HEADER_LEN)
        .map_err(|_| Error::Allocation)?;
    e.bytes.resize(HEADER_LEN, 0);
    match content {
        Content::Properties(p) => e.properties(p, 0)?,
        other => {
            e.number(1)?;
            e.blob(b"content")?;
            match other {
                Content::Text(v) => {
                    e.put(&[6])?;
                    e.blob(v.as_bytes())?;
                }
                Content::Bytes(v) => {
                    e.put(&[7])?;
                    e.blob(v)?;
                }
                Content::Encoded {
                    representation,
                    bytes,
                } => {
                    e.put(&[10])?;
                    e.blob(representation.as_bytes())?;
                    e.blob(bytes)?;
                }
                Content::Properties(_) => unreachable!(),
            }
        }
    }
    e.bytes
        .try_reserve(TRAILER_LEN)
        .map_err(|_| Error::Allocation)?;
    e.bytes.resize(e.bytes.len() + TRAILER_LEN, 0);
    Ok(e.bytes)
}

struct Encoder {
    bytes: Vec<u8>,
    limit: u64,
    max_depth: u32,
}
impl Encoder {
    fn put(&mut self, bytes: &[u8]) -> Result<(), Error> {
        let used = (self.bytes.len() - HEADER_LEN) as u64;
        if used
            .checked_add(bytes.len() as u64)
            .is_none_or(|v| v > self.limit)
        {
            return Err(Error::TooLarge);
        }
        self.bytes
            .try_reserve(bytes.len())
            .map_err(|_| Error::Allocation)?;
        self.bytes.extend_from_slice(bytes);
        Ok(())
    }
    fn number(&mut self, value: u64) -> Result<(), Error> {
        self.put(&value.to_le_bytes())
    }
    fn blob(&mut self, bytes: &[u8]) -> Result<(), Error> {
        self.number(bytes.len() as u64)?;
        self.put(bytes)
    }
    fn properties(&mut self, props: &Properties, depth: u32) -> Result<(), Error> {
        self.number(props.len() as u64)?;
        for (key, value) in props {
            self.blob(key.as_bytes())?;
            self.value(value, depth)?;
        }
        Ok(())
    }
    fn value(&mut self, value: &Value, depth: u32) -> Result<(), Error> {
        match value {
            Value::Null => self.put(&[0]),
            Value::Bool(v) => self.put(&[1, u8::from(*v)]),
            Value::I64(v) => {
                self.put(&[2])?;
                self.put(&v.to_le_bytes())
            }
            Value::U64(v) => {
                self.put(&[3])?;
                self.number(*v)
            }
            Value::F32Bits(v) => {
                self.put(&[4])?;
                self.put(&v.to_le_bytes())
            }
            Value::F64Bits(v) => {
                self.put(&[5])?;
                self.number(*v)
            }
            Value::Text(v) => {
                self.put(&[6])?;
                self.blob(v.as_bytes())
            }
            Value::Bytes(v) => {
                self.put(&[7])?;
                self.blob(v)
            }
            Value::List(v) => {
                if depth >= self.max_depth {
                    return Err(Error::TooDeep);
                }
                self.put(&[8])?;
                self.number(v.len() as u64)?;
                for value in v {
                    self.value(value, depth + 1)?;
                }
                Ok(())
            }
            Value::Properties(v) => {
                if depth >= self.max_depth {
                    return Err(Error::TooDeep);
                }
                self.put(&[9])?;
                self.properties(v, depth + 1)
            }
            Value::Encoded {
                representation,
                bytes,
            } => {
                self.put(&[10])?;
                self.blob(representation.as_bytes())?;
                self.blob(bytes)
            }
        }
    }
}

pub(crate) fn decode(bytes: &[u8], offset: u64) -> Result<Properties, Error> {
    let mut d = Decoder {
        bytes,
        at: 0,
        offset,
    };
    let properties = d.properties(0)?;
    if d.at != bytes.len() {
        return Err(d.bad("trailing property bytes"));
    }
    Ok(properties)
}
struct Decoder<'a> {
    bytes: &'a [u8],
    at: usize,
    offset: u64,
}
impl<'a> Decoder<'a> {
    fn bad(&self, reason: &'static str) -> Error {
        Error::Corruption {
            offset: self.offset + self.at as u64,
            reason,
        }
    }
    fn take(&mut self, length: usize) -> Result<&'a [u8], Error> {
        let end = self
            .at
            .checked_add(length)
            .ok_or_else(|| self.bad("property length overflow"))?;
        let value = self
            .bytes
            .get(self.at..end)
            .ok_or_else(|| self.bad("truncated property value"))?;
        self.at = end;
        Ok(value)
    }
    fn byte(&mut self) -> Result<u8, Error> {
        Ok(self.take(1)?[0])
    }
    fn number(&mut self) -> Result<u64, Error> {
        Ok(u64::from_le_bytes(
            self.take(8)?.try_into().expect("eight bytes"),
        ))
    }
    fn blob(&mut self) -> Result<Vec<u8>, Error> {
        let length = self.number()?;
        let length = usize::try_from(length)
            .map_err(|_| self.bad("property length exceeds address space"))?;
        let data = self.take(length)?;
        let mut bytes = Vec::new();
        bytes.try_reserve(length).map_err(|_| Error::Allocation)?;
        bytes.extend_from_slice(data);
        Ok(bytes)
    }
    fn text(&mut self) -> Result<String, Error> {
        String::from_utf8(self.blob()?).map_err(|_| self.bad("invalid UTF-8"))
    }
    fn properties(&mut self, depth: u32) -> Result<Properties, Error> {
        let count = self.number()?;
        if count > ((self.bytes.len() - self.at) / 9) as u64 {
            return Err(self.bad("impossible property count"));
        }
        let mut values = Vec::new();
        values
            .try_reserve(count as usize)
            .map_err(|_| Error::Allocation)?;
        for _ in 0..count {
            let key = self.text()?;
            let value = self.value(depth)?;
            values.push((key, value));
        }
        Ok(values)
    }
    fn value(&mut self, depth: u32) -> Result<Value, Error> {
        Ok(match self.byte()? {
            0 => Value::Null,
            1 => match self.byte()? {
                0 => Value::Bool(false),
                1 => Value::Bool(true),
                _ => return Err(self.bad("invalid boolean")),
            },
            2 => Value::I64(i64::from_le_bytes(
                self.take(8)?.try_into().expect("eight bytes"),
            )),
            3 => Value::U64(self.number()?),
            4 => Value::F32Bits(u32::from_le_bytes(
                self.take(4)?.try_into().expect("four bytes"),
            )),
            5 => Value::F64Bits(self.number()?),
            6 => Value::Text(self.text()?),
            7 => Value::Bytes(self.blob()?),
            8 => {
                if depth >= FORMAT_DEPTH {
                    return Err(self.bad("native nesting exceeds format limit"));
                }
                let count = self.number()?;
                if count > (self.bytes.len() - self.at) as u64 {
                    return Err(self.bad("impossible list count"));
                }
                let mut values = Vec::new();
                values
                    .try_reserve(count as usize)
                    .map_err(|_| Error::Allocation)?;
                for _ in 0..count {
                    values.push(self.value(depth + 1)?);
                }
                Value::List(values)
            }
            9 => {
                if depth >= FORMAT_DEPTH {
                    return Err(self.bad("native nesting exceeds format limit"));
                }
                Value::Properties(self.properties(depth + 1)?)
            }
            10 => Value::Encoded {
                representation: self.text()?,
                bytes: self.blob()?,
            },
            _ => return Err(self.bad("unknown value tag")),
        })
    }
}
