use crate::{N_UUID_BYTES, Result};

pub fn uuidv4() -> Result<String> {
    let mut bytes = crate::random::random_bytes()?;

    // ver: 第6オクテットの上位4bitをバージョンの数字(4)にする。
    bytes[6] &= 0x0f; // まず0にして、
    bytes[6] |= b'4'; // 上位をバージョン番号にする。

    // var:第8オクテットの上位2bitを0b10にする。
    bytes[8] &= 0x3f;
    bytes[8] |= 0x80;

    bytes_to_uuid_string(bytes)
}

fn bytes_to_uuid_string(bytes: [u8; N_UUID_BYTES]) -> Result<String> {
    const HEX: &[u8; 16] = b"0123456789abcdef";

    let mut buf = [0u8; 36];
    let mut current: usize = 0;

    for (i, byte) in bytes.iter().copied().enumerate() {
        if matches!(i, 4 | 6 | 8 | 10) {
            buf[current] = b'-';
            current += 1;
        }

        buf[current] = HEX[(byte >> 4) as usize];
        buf[current + 1] = HEX[(byte & 0x0f) as usize];
        current += 2
    }

    Ok(String::from_utf8(buf.to_vec())?)
}
