use std::fmt::Debug;

use bukrs_core::{BukrsType, BukrsNativeType};
use bytes::{BytesMut, BufMut, Buf};
use serde::{Serialize, Deserialize};

use crate::{API, net::{BukrsReqModifyInvList, BukrsResModifyInvList}};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum InventorySize {
    Inv9,
    Inv18,
    Inv27,
    Inv36,
    Inv45,
    Inv54
}

impl BukrsType for InventorySize {
    fn decode(bytes: &mut BytesMut) -> Self {
        let size = bytes.get_u8();
        match size {
            9 => InventorySize::Inv9,
            18 => InventorySize::Inv18,
            27 => InventorySize::Inv27,
            36 => InventorySize::Inv36,
            45 => InventorySize::Inv45,
            54 => InventorySize::Inv54,
            _ => panic!("Invalid inventory size")
        }
    }

    fn encode(&self, bytes: &mut BytesMut) {
        match self {
            InventorySize::Inv9 => bytes.put_u8(9),
            InventorySize::Inv18 => bytes.put_u8(18),
            InventorySize::Inv27 => bytes.put_u8(27),
            InventorySize::Inv36 => bytes.put_u8(36),
            InventorySize::Inv45 => bytes.put_u8(45),
            InventorySize::Inv54 => bytes.put_u8(54)
        }
    }

    fn ty(&self) -> BukrsNativeType {
        BukrsNativeType::U8
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InvfxId(pub u32);

impl BukrsType for InvfxId {
    fn ty(&self) -> BukrsNativeType {
        BukrsNativeType::U32
    }

    fn encode(&self, bytes: &mut BytesMut) {
        bytes.put_u32(self.0);
    }

    fn decode(bytes: &mut BytesMut) -> Self {
        InvfxId(bytes.get_u32())
    }
}

#[typetag::serde(tag = "type")]
pub trait InvFxComponent: Debug {  }    // Component

#[derive(Serialize, Deserialize, Debug)]
pub struct InvFx {
    components: Vec<Box<dyn InvFxComponent>>    // List of Components
}

// InvList

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InvList {
    pub id: InvfxId,
    pub data: Vec<InvSlot>,
}

impl InvList {
    pub async fn update(&mut self, api: &mut API, vec: Vec<InvSlot>) {
        self.data = vec;
        let BukrsResModifyInvList {  } = api.send_packet_await(BukrsReqModifyInvList { inv_id: self.id.clone(), list: self.clone() }).await.unwrap();
    }

    pub fn get(&self) -> &Vec<InvSlot> {
        &self.data
    }
}

#[typetag::serde]
impl InvFxComponent for InvList {

}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemStack {
    name: String,
    material: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InvSlot {
    slot: u8,
    item: ItemStack
}

impl BukrsType for ItemStack {
    fn decode(bytes: &mut BytesMut) -> Self {
        let name = String::decode(bytes);
        let material = String::decode(bytes);
        ItemStack { name, material }
    }

    fn encode(&self, bytes: &mut BytesMut) {
        self.name.encode(bytes);
        self.material.encode(bytes);
    }

    fn ty(&self) -> BukrsNativeType {
        BukrsNativeType::CUSTOM
    }
}

impl BukrsType for InvSlot {
    fn decode(bytes: &mut BytesMut) -> Self {
        let slot = u8::decode(bytes);
        let item = ItemStack::decode(bytes);
        InvSlot { slot, item }
    }

    fn encode(&self, bytes: &mut BytesMut) {
        self.slot.encode(bytes);
        self.item.encode(bytes);
    }

    fn ty(&self) -> BukrsNativeType {
        BukrsNativeType::CUSTOM
    }
}

impl BukrsType for InvList {
    fn decode(bytes: &mut BytesMut) -> Self {
        let id = InvfxId::decode(bytes);
        let data = Vec::<InvSlot>::decode(bytes);
        InvList { id, data }
    }

    fn encode(&self, bytes: &mut BytesMut) {
        self.id.encode(bytes);
        self.data.encode(bytes);
    }

    fn ty(&self) -> BukrsNativeType {
        BukrsNativeType::CUSTOM
    }
}

/// Convert slot to cartesian coordinates
pub fn slot_2_xy(slot: u8) -> (u8, u8) {
    let x = (slot & 0b1111) + 1;
    let y = (slot >> 4) + 1;

    (x, y)
}

pub fn xy_2_slot(xy: (u8, u8)) -> u8 {
    let x = xy.0 - 1;
    let y = xy.1 - 1;

    (y << 4) | x
}

#[cfg(test)]
mod tests {
    use super::slot_2_xy;

    #[test]
    fn test_slot_conversion() {
        println!("{:?}", slot_2_xy(0b1011000))
    }
}