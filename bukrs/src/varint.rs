// https://wiki.vg/Protocol

use bytes::{BytesMut, BufMut, Buf};
use anyhow::anyhow;

pub static SEGMENT_BITS: i32 = 0x7F; // 2^7 - 1
pub static CONTINUE_BIT: i32 = 0x80; // 2^7

pub fn write_varint(value: i32, buffer: &mut BytesMut) {
    let mut latest_value = value;   // This value is updated every loop

    loop {
        if (latest_value & !SEGMENT_BITS) == 0 {   // if value < 128
            buffer.put_u8(latest_value as u8); // value can be coerced to a u8 type
            return; // Done.
        }

        buffer.put_u8(((latest_value & SEGMENT_BITS) | CONTINUE_BIT) as u8);   // value can be coerced to a u8 type

        latest_value = ((value as u32) >> 7) as i32; // 'unsigned shift right' ( >>> )
    }
}

pub fn read_varint(src: &mut BytesMut) -> anyhow::Result<i32> {
    let mut value = 0;
    let mut position = 0;
    let mut current_byte;

    loop {
        current_byte = src.get_u8() as i32;
        value |= (current_byte & SEGMENT_BITS) << position;

        if (current_byte & CONTINUE_BIT) == 0 { break; }

        position += 7;

        if position >= 32 { 
            return Err(anyhow!("Varint Too Big"));
        }
    }

    Ok(value)
}



#[cfg(test)]
mod tests {
    use bytes::BytesMut;
    use super::{write_varint, read_varint};

    #[test]
    fn test_varint() {
        let mut buffer = BytesMut::with_capacity(1024); // Setup Buffer
        let num = 12589; // any random i32

        write_varint(num, &mut buffer);  // Encode to Buffer
        let value = read_varint(&mut buffer).expect("Uh Oh...");    // Decode Buffer
        assert_eq!(value, num);
    }
}