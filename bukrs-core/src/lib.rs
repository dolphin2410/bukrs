use bytes::{BytesMut, BufMut, Buf};

pub enum BukrsNativeType {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    VECTOR,
    STRING,
    CUSTOM
}

pub trait BukrsPacket {
    fn id(&self) -> String;
    fn encode(&self, bytes: &mut BytesMut);
}

pub trait BukrsDecodable {
    fn decode(bytes: &mut BytesMut) -> Self;
}

pub trait BukrsType {
    fn ty(&self) -> BukrsNativeType;

    fn encode(&self, bytes: &mut BytesMut);

    fn decode(bytes: &mut BytesMut) -> Self;
}


impl BukrsType for u8 {
    fn ty(&self) -> BukrsNativeType {
        BukrsNativeType::U8
    }

    fn encode(&self, bytes: &mut BytesMut) {
        bytes.put_u8(self.clone());
    }

    fn decode(bytes: &mut BytesMut) -> Self {
        bytes.get_u8()
    }
}

impl BukrsType for u16 {
    fn ty(&self) -> BukrsNativeType {
        BukrsNativeType::U16
    }

    fn encode(&self, bytes: &mut BytesMut) {
        bytes.put_u16(self.clone());
    }

    fn decode(bytes: &mut BytesMut) -> Self {
        bytes.get_u16()
    }
}

impl BukrsType for u32 {
    fn ty(&self) -> BukrsNativeType {
        BukrsNativeType::U32
    }

    fn encode(&self, bytes: &mut BytesMut) {
        bytes.put_u32(self.clone());
    }

    fn decode(bytes: &mut BytesMut) -> Self {
        bytes.get_u32()
    }
}

impl BukrsType for u64 {
    fn ty(&self) -> BukrsNativeType {
        BukrsNativeType::U64
    }

    fn encode(&self, bytes: &mut BytesMut) {
        bytes.put_u64(self.clone());
    }

    fn decode(bytes: &mut BytesMut) -> Self {
        bytes.get_u64()
    }
}

impl BukrsType for i8 {
    fn ty(&self) -> BukrsNativeType {
        BukrsNativeType::I8
    }

    fn encode(&self, bytes: &mut BytesMut) {
        bytes.put_i8(self.clone());
    }

    fn decode(bytes: &mut BytesMut) -> Self {
        bytes.get_i8()
    }
}

impl BukrsType for i16 {
    fn ty(&self) -> BukrsNativeType {
        BukrsNativeType::I16
    }

    fn encode(&self, bytes: &mut BytesMut) {
        bytes.put_i16(self.clone());
    }

    fn decode(bytes: &mut BytesMut) -> Self {
        bytes.get_i16()
    }
}

impl BukrsType for i32 {
    fn ty(&self) -> BukrsNativeType {
        BukrsNativeType::I32
    }

    fn encode(&self, bytes: &mut BytesMut) {
        bytes.put_i32(self.clone());
    }

    fn decode(bytes: &mut BytesMut) -> Self {
        bytes.get_i32()
    }
}

impl BukrsType for i64 {
    fn ty(&self) -> BukrsNativeType {
        BukrsNativeType::I64
    }

    fn encode(&self, bytes: &mut BytesMut) {
        bytes.put_i64(self.clone());
    }

    fn decode(bytes: &mut BytesMut) -> Self {
        bytes.get_i64()
    }
}

impl BukrsType for f32 {
    fn ty(&self) -> BukrsNativeType {
        BukrsNativeType::F32
    }

    fn encode(&self, bytes: &mut BytesMut) {
        bytes.put_f32(self.clone());
    }

    fn decode(bytes: &mut BytesMut) -> Self {
        bytes.get_f32()
    }
}

impl BukrsType for f64 {
    fn ty(&self) -> BukrsNativeType {
        BukrsNativeType::F64
    }

    fn encode(&self, bytes: &mut BytesMut) {
        bytes.put_f64(self.clone());
    }

    fn decode(bytes: &mut BytesMut) -> Self {
        bytes.get_f64()
    }
}

impl BukrsType for String {
    fn ty(&self) -> BukrsNativeType {
        BukrsNativeType::STRING
    }

    fn encode(&self, bytes: &mut BytesMut) {
        let str_bytes = self.as_bytes();
        bytes.put_u32(str_bytes.len() as u32);
        bytes.put(str_bytes);
    }

    fn decode(bytes: &mut BytesMut) -> Self {
        let len = bytes.get_u32() as usize;
        let str_bytes = (0..len).map(|_|bytes.get_u8()).collect::<Vec<u8>>();
        String::from_utf8(str_bytes).unwrap()
    }
}

impl <T> BukrsType for Vec<T> where T: BukrsType {
    fn ty(&self) -> BukrsNativeType {
        BukrsNativeType::VECTOR
    }

    fn encode(&self, bytes: &mut BytesMut) {
        bytes.put_u32(self.len() as u32);
        for item in self.iter() {
            item.encode(bytes);
        }
    }

    fn decode(bytes: &mut BytesMut) -> Self {
        let size = bytes.get_u32();
        let mut vec = vec![];
        for _ in 0..size {
            let item = T::decode(bytes);
            vec.push(item);
        }
        vec
    }
}