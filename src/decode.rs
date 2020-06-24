use crate::{data, data::Data, error::ProtodecError, key};

pub fn decode_message(buffer: &[u8]) -> Result<Vec<Data>, ProtodecError> {
    let mut result: Vec<Data> = vec![];

    if buffer.len() == 0 {
        return Ok(result);
    }

    let mut buf = buffer;

    loop {
        let (key, rest) = key::take_key(buf);

        let (data, rest) = data::take_data(key, rest)?;

        buf = rest;

        result.push(data);

        if rest.is_empty() {
            break;
        }
    }

    Ok(result)
}
