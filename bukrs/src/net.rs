use std::{task::{Poll, Waker}, sync::{Arc, Mutex}, pin::Pin, collections::HashMap, fmt::{Debug, Display}};

use bytes::{Buf, BytesMut, BufMut};
use futures::Future;
use serde::{Serialize, Deserialize};
use tokio_util::codec::{Encoder, Decoder};
use bukrs_core::BukrsType;
use bukrs_core::{BukrsPacket,BukrsDecodable};

use crate::{varint, core::{invfx::{InventorySize, InvList, InvfxId}, player::{PlayerId, PlayerData}}, register_packet, arc_mutex};

#[ctor::ctor]
pub static CONSTRUCTORS: Arc<Mutex<HashMap<String, fn(buf: &mut BytesMut) -> Box<dyn Packet>>>> = arc_mutex!(HashMap::new());

#[typetag::serde(tag = "type")]
pub trait Packet: Send + Sync + std::any::Any + Debug + Display + BukrsPacket {
    fn clone_box(&self) -> Box<dyn Packet>;
    fn get_any(&self) -> Box<dyn std::any::Any>;
}

pub fn cast_packet<T: Packet + Clone>(packet: &Box<dyn Packet>) -> Option<T> {
    packet.get_any().downcast_ref::<T>().map(|p| p.clone())
}

register_packet! {
    BukrsReqAPI {

    }
    BukrsResAPI { 
        api_id u32
    }
}

register_packet! {
    BukrsReqOnlinePlayers { 

    }
    BukrsResOnlinePlayers { 
        players Vec<PlayerId>
    }
}

register_packet! {
    BukrsReqPlayerById { 
        player_id PlayerId
    }
    BukrsReqPlayerByName { 
        player_name String
    }
    BukrsResPlayerData { 
        data PlayerData
    }
}

register_packet! {
    BukrsReqCreateInventory { name String; size InventorySize } // Request creation invfx
    BukrsResCreateInventory { inv_id InvfxId }    // Verify Invfx creation
    BukrsSDInvClick { slot u8; player_id PlayerId }    // SD: Server Data
    BukrsSDInvOpen { player_id PlayerId }
    BukrsSDInvClose { player_id PlayerId }
    BukrsReqPlayerInvOpen { inv_id InvfxId; player_id PlayerId }
    BukrsResPlayerInvOpen {  }
    BukrsReqCreateInvList { inv_id InvfxId; list InvList }
    BukrsResCreateInvList {  }
    BukrsReqModifyInvList { inv_id InvfxId; list InvList }
    BukrsResModifyInvList {  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerResponse(pub u32);

#[derive(Serialize, Deserialize)]
pub struct BukrsPacketData { pub payload_id: Option::<u32>, pub event: Box::<dyn Packet> }

pub struct Codec;

impl Encoder<BukrsPacketData> for Codec {
    type Error = anyhow::Error;

    fn encode(&mut self, event: BukrsPacketData, dst: &mut bytes::BytesMut) -> Result<(), Self::Error> {
        let mut payload = BytesMut::with_capacity(1024);
        event.event.id().encode(&mut payload);
        event.event.encode(&mut payload);

        let payload_id = if event.payload_id.is_none() { 0 } else { event.payload_id.unwrap() };
        encode_header(dst, &payload, payload_id)?;
        dst.put(payload);
        Ok(())
    }
}

impl Decoder for Codec {
    type Error = anyhow::Error;
    type Item = BukrsPacketData;


    fn decode(&mut self, src: &mut bytes::BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let mut read_buffer = src.clone();
        if read_buffer.remaining() < 6 {
          return Ok(None);
        }

        let (remaining_length, payload_id) = decode_header(&mut read_buffer)?;
        if read_buffer.remaining() < remaining_length {
          return Ok(None);
        }
        src.advance(src.remaining() - read_buffer.remaining()); // Increment header

        let packet_name = String::decode(&mut read_buffer);

        let map = CONSTRUCTORS.lock().unwrap();

        let func = map.get(&packet_name).unwrap();

        let payload_id = if payload_id == 0 { None } else { Some(payload_id) };

        let packet = BukrsPacketData { payload_id, event: func(&mut read_buffer) };

        src.advance(remaining_length);

        Ok(Some(packet))
    }
}

fn encode_header(dst: &mut BytesMut, packet_data: &BytesMut, payload_id: u32) -> anyhow::Result<()> {
    varint::write_varint(i32::try_from(packet_data.len())?, dst);
    dst.put_u32(payload_id);

    Ok(())
}

fn decode_header(src: &mut BytesMut) -> anyhow::Result<(usize, u32)> {
    let packet_size = usize::try_from(varint::read_varint(src)?)?; // packet size excluding header
    let payload_id = src.get_u32(); 
    Ok((packet_size, payload_id))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use bytes::BytesMut;
    use tokio_util::codec::{Decoder, Encoder};

    use crate::{net::BukrsPacketData, arc_mutex, core::invfx::{InvfxId, InvList}};

    use super::{Codec, BukrsFuture, BukrsReqCreateInvList};

    #[test]
    fn codec_test() {
        let mut buf = BytesMut::with_capacity(1024);

        let mut codec = Codec;
        codec.encode(BukrsPacketData { payload_id: Some(1024), event: Box::new(BukrsReqCreateInvList { inv_id: InvfxId(1024), list: InvList { id: InvfxId(1024), data: vec![] } }) }, &mut buf).unwrap();
        if let Some(item) = codec.decode(&mut buf).unwrap() {
            println!("{}", item.event);
        }
    }

    #[tokio::test]
    async fn futures_test() {
        let future = BukrsFuture::new(1024, arc_mutex!(HashMap::new()));
        (&future).await;
    }
}

pub struct BukrsFuture {
    pub(crate) data: Mutex<Option<Box<dyn Packet>>>,
    pub(crate) waker: Mutex<Option<Waker>>,
    pub(crate) payload_id: u32,
    pub(crate) payload_handler: Arc<Mutex<HashMap<u32, Arc<BukrsFuture>>>>,
}

impl BukrsFuture {
    pub fn new(payload_id: u32, payload_handler: Arc<Mutex<HashMap<u32, Arc<BukrsFuture>>>>) -> BukrsFuture {
        BukrsFuture { data: Mutex::new(None), waker: Mutex::new(None), payload_id, payload_handler }
    }

    pub fn set_waker(&self, waker: Waker) {
        let mut locked = self.waker.lock().unwrap();
        *locked = Some(waker);
    }

    pub fn set_data(&self, data: Box<dyn Packet>) {
        let mut locked = self.data.lock().unwrap();
        *locked = Some(data);
    }

    pub fn wake(&self) {
        self.waker.lock().unwrap().as_ref().unwrap().wake_by_ref();
    }
}

impl Future for &BukrsFuture {
    type Output = Box<dyn Packet>;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let fut = Pin::into_inner(self);
        if let Some(event) = fut.data.lock().unwrap().as_ref() {
            fut.payload_handler.lock().unwrap().remove(&fut.payload_id);
            Poll::Ready(event.clone_box())
        } else {
            let waker = cx.waker().clone();
            fut.set_waker(waker);
            Poll::Pending
        }
    }
}