use bukrs_core::{BukrsType, BukrsNativeType};
use bytes::{BytesMut, BufMut, Buf};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlayerId(pub u32);

impl BukrsType for PlayerId {
    fn ty(&self) -> BukrsNativeType {
        BukrsNativeType::U32
    }

    fn encode(&self, bytes: &mut BytesMut) {
        bytes.put_u32(self.0);
    }

    fn decode(bytes: &mut BytesMut) -> Self {
        PlayerId(bytes.get_u32())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UUID {
    lsb: u64,
    msb: u64
}

impl BukrsType for UUID {
    fn decode(bytes: &mut BytesMut) -> Self {
        let lsb = bytes.get_u64();
        let msb = bytes.get_u64();
        UUID { lsb, msb }
    }

    fn encode(&self, bytes: &mut BytesMut) {
        bytes.put_u64(self.lsb);
        bytes.put_u64(self.msb);
    }

    fn ty(&self) -> BukrsNativeType {
        BukrsNativeType::CUSTOM
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlayerData {
    pub id: PlayerId,
    pub name: String,
    pub uuid: UUID
}

impl BukrsType for PlayerData {
    fn decode(bytes: &mut BytesMut) -> Self {
        let id = PlayerId::decode(bytes);
        let name = String::decode(bytes);
        let uuid = UUID::decode(bytes);
        PlayerData { id, name, uuid }
    }

    fn encode(&self, bytes: &mut BytesMut) {
        self.id.encode(bytes);
        self.name.encode(bytes);
        self.uuid.encode(bytes);
    }

    fn ty(&self) -> BukrsNativeType {
        BukrsNativeType::CUSTOM
    }
}

