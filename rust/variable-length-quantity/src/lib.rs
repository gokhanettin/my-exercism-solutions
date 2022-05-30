#[derive(Debug, PartialEq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    values
        .iter()
        .flat_map(|value| {
            let mut bytes = Vec::new();
            let mut tmp = *value;
            if tmp < (1 << 7) {
                bytes.push(tmp as u8);
            } else {
                let mut n = 0;
                while tmp > 0 {
                    let mut byte = (tmp & 0x7F) as u8;
                    if n > 0 {
                        byte |= 0x80;
                    }
                    tmp >>= 7;
                    n += 1;
                    bytes.push(byte);
                }
            }
            bytes.into_iter().rev()
        })
        .collect()
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut values = Vec::new();
    let mut value = 0;
    for byte in bytes {
        value |= (byte & 0x7F) as u32;
        if byte & 0x80 != 0 {
            if value.leading_zeros() < 7 {
                return Err(Error::Overflow);
            }
            value <<= 7;
        } else {
            values.push(value);
            value = 0;
        }
    }
    if values.is_empty() {
        return Err(Error::IncompleteNumber);
    }
    Ok(values)
}
